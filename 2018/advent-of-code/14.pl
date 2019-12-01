#!/usr/bin/env perl6
use v6.c;


sub step(@lst, $a is rw, $b is rw, $next is rw) {
    my $x = @lst[$a] + @lst[$b];
    if $x >= 10 {
        @lst[$next] = $x div 10;
        @lst[$next+1] = $x % 10;
        $next += 2;
    } else {
        @lst[$next] = $x;
        $next += 1;
    }
    $a = ($a + @lst[$a] + 1) % $next;
    $b = ($b + @lst[$b] + 1) % $next;
}

sub ends-at(@lst, @gen, $next) {
    for 1..@gen.elems {
        return False if @lst[$next-$_] != @gen[*-$_];
    }
    return True
}

sub MAIN(Int $gen = 505961, Int $target = 505961) {
    my int32 @lst[100*$gen];
    @lst[0] = 3;
    @lst[1] = 7;
    my ($a, $b, $next) = 0, 1, 2;

    while $next < $gen+10 {
        step(@lst, $a, $b, $next);
    }

    put ($gen..^$gen+10).map({ @lst[$_] }).join("");

    my int32 @gen = (~$target).comb.map: +*;
    # Test completed so far:
    for @gen.elems..$next {
        if ends-at(@lst, @gen, $_) {
            put "Found $target after recipe { $_ - @gen.elems }";
            exit;
        }
    }

    loop {
        my $last = $next;
        step(@lst, $a, $b, $next);
        for $last^..$next {
            if ends-at(@lst, @gen, $_) {
                put "Found $target after recipe { $_ - @gen.elems }";
                exit;
            }
        }
    }
}
