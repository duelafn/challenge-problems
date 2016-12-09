#!/usr/bin/env perl6
use v6.c;

sub MAIN($file="08.in", Int :$rows=6, Int :$cols=50, Bool :$verbose) {

    my @screen = (1..$rows).map: { Array.new('.' xx $cols) };
    for $file.IO.lines {
        when m:s/rect (\d+)x(\d+)/ {
            for ^$0 -> $x {
                for ^$1 -> $y {
                    @screen[$y][$x] = '*';
                }
            }
        }

        when m:s/rotate row y\=(\d+) by (\d+)/ {
            @screen[$0] .= rotate(-$1);# rotate() is backward from what I'd expect
        }

        when m:s/rotate column x\=(\d+) by (\d+)/ {
            my $brk = $rows - $1;
            # Oddly, "for eager flat($brk..^$rows, ^$brk).map({ @screen[$_][$0] }).kv -> ..."
            # doesn't work (list still evaluaed lazily, so items get oeverwritten before caching).
            my @tmp = flat($brk..^$rows, ^$brk).map({ @screen[$_][$0] });
            for @tmp.kv -> $i, $val {
                @screen[$i][$0] = $val;
            }
        }

        default {
            die "Can't parse $_";
        }

        NEXT {
            if $verbose {
                put "$_:";
                @screen.map: *.put;
            }
        }
    }

    @screen.map: *.put;
    say [+] @screen».grep: { /"*"/ };
}


# Bummer, get this error for most commands in here:
#   """Partially dimensioned views of arrays not yet implemented. Sorry."""
sub xxMAIN($file="08.in", Int :$rows=6, Int :$cols=50) {
    my int @screen[$rows;$cols] = (0 xx $cols) xx $rows;
    for $file.IO.lines {
        when m:s/rect (\d+)x(\d+)/ {
            @screen[^$1;^$0] = 1 xx ($1 * $0);
        }
        when m:s/rotate row y\=(\d+) by (\d+)/ {
            my $brk = $cols - $1;
            @screen[$0;*] = @screen[$0;$brk..^$cols, ^$brk];
        }
        when m:s/rotate column x\=(\d+) by (\d+)/ {
            my $brk = $rows - $1;
            @screen[*;$0] = @screen[$brk..^$rows, ^$brk;$0];
        }
        default {
            die "Can't parse $_";
        }
    }

    @screen[$_;*].put for ^$rows;
    say [+] @screen;
}
