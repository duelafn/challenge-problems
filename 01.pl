#!/usr/local/bin/perl6
use v6.c;

class Walker {
    has $.x = 0;
    has $.y = 0;
    has $.vec = (0, 1);

    multi method turn(Str() $dir) {
        given $dir {
            when 'L' { $!vec = (-$!vec[1], $!vec[0]) }
            when 'R' { $!vec = ( $!vec[1],-$!vec[0]) }
            default  { die "Invalid turn: $dir" }
        }
    }

    multi method walk(Int() $length) {
        $!x += $length * $!vec[0];
        $!y += $length * $!vec[1];
    }

    method pos()      { return "($!x, $!y)" }
    method distance() { return abs($!x) + abs($!y) }
}


sub MAIN($input) {
    my Walker $walker .= new;
    my %seen = ( "(0, 0)" => 1 );# Just in case we return home first!
    for slurp($input).comb(/ (<[LR]>) (\d+) /, :match)».Slip -> [$dir, $step] {
        $walker.turn($dir);
        for ^$step {
            $walker.walk(1);
            if %seen{$walker.pos}:exists {
                say "Hmm, this place looks familiar [ $walker.pos() at distance $walker.distance() ]";
            } else {
                %seen{$walker.pos} = 1;
            }
        }
    }
    say "SatNav says we are $walker.distance() blocks from home";
}
