#!/usr/bin/env perl6
use v6.c;

sub fingerprint($state, $left) {
    [+] (0..^$state.chars).map({ '#' eq substr($state, $_, 1) ?? $_+$left !! 0 })
}

sub step($state is rw, $left is rw, %rule) {
    while substr($state, 0, 3) ne '...' {
        $state = '.' ~ $state;
        $left--;
    }
    $state = $state ~ '.' while substr($state, *-3)  ne '...';

    $left += 2;
    $state = (2..($state.chars-2)).map({
        %rule{substr($state, $_-2, 5)}:exists ?? %rule{substr($state, $_-2, 5)} !! '.'
    }).join("");
}

sub MAIN(IO() $file="12.in") {
    my $state;
    my %rule;

    for $file.lines -> $line {
        if $line ~~ /"initial state:" \s* (<[.#]>+)/ {
            $state = ~$0;
        } elsif $line ~~ /(<[.#]>+) " => " (<[.#]>)/ {
            %rule{~$0} = ~$1;
        }
    }

    my $left = 0;
    my $orig = $state;
    step($state, $left, %rule) for 1..20;
    put "Generation 20 fingerprint: ", fingerprint($state, $left);

    $left = 0;
    $state = $orig;
    my $cycle = 0;
    my %cache;
    loop {
        put $cycle if $cycle %% 1000;
        my $last = $state ~ $left;  # First "$last" value is $orig
        my $flast = fingerprint($state, $left);
        if %cache{$last}:exists or $cycle > 2000 {
            put "Cycle length: $cycle";
            last;
        }
        $cycle++;
        step($state, $left, %rule);
        %cache{$last} = 1;
        my $finger = fingerprint($state, $left);
        put "$cycle: $finger   Δ{ $finger - $flast }";
    }
    # 0 => 1 => 2 => 3 => 4 => 0  ::  $cycle == 5, +%cache == 5 is number of arrows

### Part 2:
# 1990: 71098   Δ36
# 1991: 71134   Δ36
# 1992: 71170   Δ36
# 1993: 71206   Δ36
# 1994: 71242   Δ36
# 1995: 71278   Δ36
# 1996: 71314   Δ36
# 1997: 71350   Δ36
# 1998: 71386   Δ36
# 1999: 71422   Δ36
# 2000: 71458   Δ36
# $ DeanUtil '71458+36*(50000000000-2000)'
# 1799999999458

#     # my $GOAL = 50000000000;
#     my $GOAL = 750;
#
#     if $GOAL < 1000 {
#         $left = 0;
#         $state = $orig;
#         step($state, $left, %rule) for 1..$GOAL;
#         put "Generation $GOAL fingerprint: ", fingerprint($state, $left), " (real)";
#     }
#
#     $left = 0;
#     $state = $orig;
#     step($state, $left, %rule) for 1..($GOAL % $cycle);
#     put "Generation $GOAL fingerprint: ", fingerprint($state, $left);
}
