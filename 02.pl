#!/usr/bin/env perl6
use v6.c;

class Pad {
    has $.x = 0;
    has $.y = 0;
    has @.pad;

    method move(Str $dir) {
        given ($dir) {
            when "U" { $!y -= 1 if $!y > 0              and @!pad[$!y-1][$!x] ne '.' }
            when "D" { $!y += 1 if $!y < @!pad.end      and @!pad[$!y+1][$!x] ne '.' }
            when "L" { $!x -= 1 if $!x > 0              and @!pad[$!y][$!x-1] ne '.' }
            when "R" { $!x += 1 if $!x < @!pad[$!y].end and @!pad[$!y][$!x+1] ne '.' }
        }
    }

    method get_key() {
        return @!pad[$!y][$!x]
    }
}

my @ENVISIONED_PAD = "
123
456
789
".comb(/\S/).rotor(3);

my @ACTUAL_PAD = Q:to/ACTUAL/.comb(/\S/).rotor(5);
. . 1 . .
. 2 3 4 .
5 6 7 8 9
. A B C .
. . D . .
ACTUAL

sub MAIN() {
#     my $pad = Pad.new( :x(1), :y(1), pad => @ENVISIONED_PAD );
    my $pad = Pad.new( :x(0), :y(3), pad => @ACTUAL_PAD );
    for "02.in".IO.lines -> $line {
        for $line.comb(/<[UDLR]>/) -> $step {
            $pad.move($step);
        }
        print $pad.get_key;
    }
    print "\n";
}
