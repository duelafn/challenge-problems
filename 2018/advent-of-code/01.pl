#!/usr/bin/env perl6
use v6.d.PREVIEW;

sub MAIN(IO() $file="01.in") {
    my @vec = $file.lines().map: +*;
    my $f = 0;
    my %seen;

    my $i = 0; my $loops = 0;
    while !%seen{$f} {
        %seen{$f} = True;
        $f += @vec[$i++];
        if $i >= @vec.elems {
            put "Frequency after 1 full loop: $f" if 0 == $loops++;
            $i = 0;
        }
    }
    put "Frequency $f repeated after $loops loops and $i items";
}
