#!/usr/bin/env perl6
use v6.c;


sub MAIN(IO() $file="20.in") {
    my @ranges = gather for $file.lines {
        take Range.new(+$0, +$1) if / ^ (\d+) "-" (\d+) $ /;
    }

    my @blocks;
    for @ranges.sort({ $^r.min }) -> $r {
        if @blocks and $r.min <= @blocks[*-1].max + 1 {
            @blocks[*-1] = Range.new(@blocks[*-1].min, $r.max) if $r.max !~~ @blocks[*-1]
        } else {
            @blocks.append: $r;
        }
    }

    put "Minimum accepted: { @blocks[0].max + 1 }";
    put "Number  accepted: { 4294967296 - [+] @blocks».elems }";
}
