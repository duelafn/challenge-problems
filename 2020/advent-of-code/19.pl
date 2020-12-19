#!/usr/bin/perl
use warnings; use strict; use 5.020;
use Method::Signatures;
use Marpa::R2;

func load($fname, $part2=undef) {
    my @words;
    my $rules = << '__G__';
lexeme default = latm => 1
:default ::= action => [name,values]
:start   ::= r0
__G__
# r102 ::= r16 r116 | r26 r132
# r6: r44 r44
# ...
# r16  ~ 'b'
# ...

    open my $FH, "<", $fname or die "Error reading $fname: $!";
    for (<$FH>) {
        chomp;
        if (s/^(\d+):\s*//) {
            my $r = "r$1";
            $_ = '42 | 42 8'        if $part2 and $r eq 'r8';
            $_ = '42 31 | 42 11 31' if $part2 and $r eq 'r11';

            if (s/(\d+)/r$1/g) {
                $rules .= "$r ::= $_\n";
            } else {
                s/[^a-z]//g;
                $rules .= "$r ~ '$_'\n";
            }
        }

        elsif (/\S/) {
            push @words, $_;
        }
    }

    my $grammar = Marpa::R2::Scanless::G->new({ source => \$rules });
    return ($grammar, \@words);
}

{
    my ($grammar, $words) = load($ARGV[0] // '19.in');
    my $n = grep eval{ $grammar->parse(\$_) }, @$words;
    say "Part 1: $n matched";
}
{
    my ($grammar, $words) = load($ARGV[0] // '19.in', 'part2');
    my $n = grep eval{ $grammar->parse(\$_) }, @$words;
    say "Part 2: $n matched";
}
