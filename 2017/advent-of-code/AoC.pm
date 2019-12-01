package AoC;
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Dean::Util qw/ rofh /;

use parent "Exporter";
our %EXPORT_TAGS = (
    matrix => [qw/ read_matrix /],
    file   => [qw/ lines readfile /],
    vm     => [qw/ $re_int $re_name $re_cmp $re_binop /],
    misc   => [qw/ knot_hash /],
);
our @EXPORT_OK = map @$_, values %EXPORT_TAGS;
$EXPORT_TAGS{all} = \@EXPORT_OK;


# UNARY
#   inc, dec:  increment or decrement register by 1
#
# BINARY
#   inc, dec:  add/subtract values and update register (first arg)
#   add, sub:  add/subtract values
#
# COMPARISON
#   >, <, >=, <=, ==, !=

our $re_int   = qr/-?\d+/;
our $re_name  = qr/[a-zA-Z]\w*/;
our $re_cmp   = qr/(?:>=?|<=?|!=|==)/;
# our $re_binop = qr/(?:inc|dec|add|sub)/;
our $re_binop = qr/(?:inc|dec|snd|rcv)/;
our $re_uniop = qr/(?:set|inc|dec|add|sub|mul|mod|jgz)/;

package VM {
    use Moose;
    use MooseX::StrictConstructor;
    use Method::Signatures::Simple;
    use Data::Dump 'pp';

    has lineno => (
        is         => 'rw',
        isa        => 'Int',
        # perldoc Moose::Meta::Attribute::Native::Trait::Counter
        traits     => ['Counter'],
        default    => 0,
        handles    => {
            next_line => 'inc',
            prev_line => 'dec',
        }
    );

    has msgs => (
        is         => 'ro',
        isa        => 'ArrayRef',
        # perldoc Moose::Meta::Attribute::Native::Trait::Array
        traits     => ['Array'],
        default    => sub { [] },
        handles    => {
            push_msg => 'push',
            has_msgs => 'count',
            next_msg => 'shift',
        },
    );

    has reg => (
        is         => 'ro',
        isa        => 'HashRef',
        # perldoc Moose::Meta::Attribute::Native::Trait::Hash
        traits     => ['Hash'],
        default    => sub { {} },
        handles    => {
            # get($name, $dflt) implemented below
            set             => "set",
            registers       => "elements",
            register_names  => "keys",
            register_values => "values",
            has_register    => "exists",
        },
    );
    method get($name, $dflt) {
        my $reg = $self->reg;
        return( $$reg{$name} //= ($dflt // 0) );
    }

    method val($v) {
        return( ($v =~ /[^\d-]/) ? $self->get($v) : $v );
    }

    method dump_registers() {
        say pp $self->reg;
    }

    method test($op, $a, $b) {
        if    ($op eq '>')  { return $self->val($a) >  $self->val($b); }
        elsif ($op eq '<')  { return $self->val($a) <  $self->val($b); }
        elsif ($op eq '>=') { return $self->val($a) >= $self->val($b); }
        elsif ($op eq '<=') { return $self->val($a) <= $self->val($b); }
        elsif ($op eq '==') { return $self->val($a) == $self->val($b); }
        elsif ($op eq '!=') { return $self->val($a) != $self->val($b); }
        else { die "unknown test '$op'"; }
    }

    method uniop($op, $a) {
        my $reg = $self->reg;
        if    ($op eq 'inc')  { $$reg{$a} += 1; $self->next_line; }
        elsif ($op eq 'dec')  { $$reg{$a} -= 1; $self->next_line; }
        elsif ($op eq 'snd')  { $$reg{_snd} = $self->val($a); $self->next_line; }
        elsif ($op eq 'rcv')  { exit say "RECOVER: $$reg{_snd}" if $self->val($a); $self->next_line; }
        else { die "unknown uniop '$op'"; }
    }

    method binop($op, $a, $b) {
        my $reg = $self->reg;
        if    ($op eq 'set')  { $$reg{$a}  = $self->val($b); $self->next_line; }
        elsif ($op eq 'inc')  { $$reg{$a} += $self->val($b); $self->next_line; }
        elsif ($op eq 'dec')  { $$reg{$a} -= $self->val($b); $self->next_line; }
        elsif ($op eq 'add')  { $$reg{$a} += $self->val($b); $self->next_line; }
        elsif ($op eq 'sub')  { $$reg{$a} -= $self->val($b); $self->next_line; }
        elsif ($op eq 'mul')  { $$reg{$a} *= $self->val($b); $self->next_line; }
        elsif ($op eq 'mod')  { $$reg{$a} %= $self->val($b); $self->next_line; }
        elsif ($op eq 'jgz')  { $self->next_line(($self->val($a)  > 0) ? $self->val($b) : 1); }
        elsif ($op eq 'jnz')  { $self->next_line(($self->val($a) != 0) ? $self->val($b) : 1); }

        # Not binary in traditional sense, binary only because we pass the other VM.
        elsif ($op eq 'snd')  {
            $b->push_msg($self->val($a));
            $$reg{_snd_count}++;
            $self->next_line;
        }
        elsif ($op eq 'rcv')  {
            if ($self->has_msgs) {
                $$reg{$a} = $self->next_msg;
                $self->next_line;
            }
        }

        else { die "unknown binop '$op'"; }
    }

    no Moose;
    __PACKAGE__->meta->make_immutable;
}



sub knot_hash {
    my @magic = (17, 31, 73, 47, 23);
    my $str = shift;
    my @key = (map(ord($_), split //, $str), @magic);

    my $n = 256;
    my ($p, $skip, $l) = (0, 0, [0..255]);
    for (1..64) {
        for my $len (@key) {
            if ($p + $len >= @$l) {
                my @tmp = (@$l, @$l);
                my @new = reverse splice @tmp, $p, $len;
                splice @tmp, $p, 0, @new;
                @$l = (@tmp[$n..($len+$p-1)], @tmp[($len+$p-$n)..($n-1)]);
            } else {
                my @new = reverse splice @$l, $p, $len;
                splice @$l, $p, 0, @new;
            }
            $p = ($p + $len + $skip) % $n;
            $skip += 1;
        }
    }

    my $hash = "";
    $hash .= sprintf "%02x", reduce { $a ^ $b } splice @$l, 0, 16 while @$l;
    return $hash;
}



sub lines {
    my $f = shift;
    open my $F, "<", $f or die "Can't open $f for reading: $!";
    chomp(my @lines = <$F>);
    return @lines;
}

sub readfile {
    my $f = shift;
    open my $F, "<", $f or die "Can't open $f for reading: $!";
    local $/ = undef;
    return scalar <$F>;
}

sub read_matrix {
    my ($fname, %o) = @_;
    $o{one}      //= 1;
    $o{blank}    //= 1;
    $o{comments} //= 1;
    $o{sep}      //= qr/\s+/;
    my $F = ref($fname) ? $fname : rofh($fname);
    return if eof($F);
    my @M;
    while (defined($_ = <$F>)) {
        chomp;
        last if @M and $o{one} and /^\s*$/;
        next if $o{blank} and /^\s*$/;
        next if $o{comments} and /^#/;
        push @M, [ split /$o{sep}/ ];
    }
    return unless @M;
    return \@M;
}









1;
