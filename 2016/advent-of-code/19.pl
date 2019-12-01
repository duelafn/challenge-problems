#!/usr/bin/env perl6
use v6.c;

sub MAIN(Int() $algorithm=1, Int() $count=3004953) {
    my @algorithm = (
        -> $i, $len { $i + 1 },
        -> $i, $len { $i + $len div 2 },
    );
    my &func = @algorithm[$algorithm - 1];
    my @elves = 1..$count;
    my $i = 0;
    while @elves.elems > 1 {
        put "{ @elves.elems } elves standing" if @elves.elems %% 1_000;
        my $kill = func($i, @elves.elems) % @elves.elems;
        @elves.splice($kill, 1);
        $i++ unless $kill < $i;
        $i = 0 if $i > @elves.end;
    }
    .say for @elves;
}
