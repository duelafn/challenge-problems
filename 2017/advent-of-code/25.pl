#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ pairmap reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Data::Dump 'pp';
use Dean::Util qw/ cat /;
use lib '.';

use AoC ':all';
# file      # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix    # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)
# knot_hash # Day 10

# pairmap { $a, $b } $vm->registers;


package Turing {
    use Method::Signatures::Simple;
    use re 'taint'; use 5.014;

    sub new {
        my $class = shift;
        my %opt = @_;
        my $self = bless { state => "A", tape => "0", pos => 0, %opt }, $class;
        return $self;
    }

    method build($init, @states) {
        my $t = $self->new;
        die "I" unless $init =~ /Begin in state (\w+)/;
        $t->state($1);

        die "II" unless $init =~ /checksum after (\d+) steps/;
        $$t{stash}{steps} = $1;

        $t->add_state(parse_state($_)) for @states;

        return $t;
    }

    method show {
        say $$self{tape};
        say " "x($$self{pos}), "^";
    }

    method checksum() {
        0 + ($$self{tape} =~ s/1/1/g);
    }

    method step($n) {
        $n //= 1;
        $$self{states}{$$self{state}}->($self) while $n--;
    }

    method add_state($name, $sub) {
        $$self{states}{$name} = $sub;
    }

    method state($name) {
        $$self{state} = $name if $name;
        return $$self{state};
    }

    method write($v) {
        substr($$self{tape}, $$self{pos}, 1, $v);
    }

    method read($v) {
        return substr($$self{tape}, $$self{pos}, 1);
    }

    method right($n) {
        $$self{pos} += ($n // 1);
        if ($$self{pos} >= length($$self{tape})) {
            $$self{tape} .= '0'x($$self{pos} - length($$self{tape}) + 1);
        }
    }

    method left($n) {
        $$self{pos} -= ($n // 1);
        if ($$self{pos} < 0) {
            substr($$self{tape}, 0, 0, '0'x(-$$self{pos}));
            $$self{pos} = 0;
        }
    }

    sub parse_state {
        my @lines = split /\n/, shift;
        die "1" unless $lines[0] =~ /^In state (\w+):/;
        my $name = $1; shift @lines;
        my (@if, @else);

        die "2" unless $lines[0] =~ /If the current value is 0/;
        shift @lines;

        die "3" unless $lines[0] =~ /Write the value (1|0)/;
        push @else, "\$t->write($1);"; shift @lines;
        die "4" unless $lines[0] =~ /Move one slot to the (left|right)/;
        push @else, "\$t->$1();"; shift @lines;
        die "5" unless $lines[0] =~ /Continue with state (\w+)/;
        push @else, "\$t->state('$1');"; shift @lines;

        die "6" unless $lines[0] =~ /If the current value is 1/;
        shift @lines;

        die "7" unless $lines[0] =~ /Write the value (1|0)/;
        push @if, "\$t->write($1);"; shift @lines;
        die "8" unless $lines[0] =~ /Move one slot to the (left|right)/;
        push @if, "\$t->$1();"; shift @lines;
        die "9" unless $lines[0] =~ /Continue with state (\w+)/;
        push @if, "\$t->state('$1');"; shift @lines;

        my $sub = join "\n", (
            'sub {',
                'my $t = shift;',
                'if ($t->read) {',
                    @if,
                '} else {',
                    @else,
                '}',
            '}',
        );

        return ($name => eval($sub));
    }

}



func MAIN() {
    my $t;

    $t = Turing->build(split /\n\n+/, do { local $/; scalar <DATA> });
    for (1..$$t{stash}{steps}) {
        $t->show; $t->step;
    }
    $t->show;
    is $t->checksum, 3, "Test";

    $t = Turing->build(split /\n\n+/, scalar cat "25.in");
    $t->step($$t{stash}{steps});
    is $t->checksum, 3578, "Stage 1";
}



MAIN(@ARGV);
done_testing;

__DATA__
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
