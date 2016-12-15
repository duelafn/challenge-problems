#!/usr/bin/env perl6
use v6.c;
# It's a shame that this problem was so small, brute force wins out over
# being clever here, but I just had to implement using the Chinese
# Remainder Theorem.

=begin pod

=head3 xgcd

 ($alpha, $beta, $d) = xgcd($a, $b)

For a pair of integers, a and b, perform the extended Euclidean
algorithm to compute alpha, beta, and d such that:

 d = alpha * a  +  beta * b

In particular, d is the GCD of a and b and, if d = 1 then alpha = a^-1 mod b.

=end pod

sub xgcd(Int() $a, Int() $b) {
    my Int ($α0, $α1) = 1, 0;
    my Int ($β0, $β1) = 0, 1;
    my Int ($r0, $r1) = $a, $b;
    while $r1 != 0 {
        my $q = $r0 div $r1;
        ($r0, $r1) = ($r1, $r0 - $q * $r1);
        ($α0, $α1) = ($α1, $α0 - $q * $α1);
        ($β0, $β1) = ($β1, $β0 - $q * $β1);
    }
    return ($α0, $β0, $r0);
}


=begin pod

=head3 solve_modulars

 my $t = solve_modulars([ [ $value1, $mod1 ], [ $value2, $mod2 ], ... ]);

Solves a system of modular equations:

   x = $value1   (mod $mod1)
   x = $value2   (mod $mod2)
   ...

Returns a value of x (0 <= x < ∏ $mod) which satisfies the given equations.
Requires that the moduli are pairwise coprime.

That such a system is uniquely solvable is known as the "Chinese Remainder Theorem".

=end pod

sub solve_modulars(@eqns) returns Int {
    my $M = [*] @eqns.map: { .[1] }
    my (@y, @z);
    for @eqns -> [$value, $mod] {
        my $z = $M div $mod;
        @z.push: $z;
        my ($α, $β, $gcd) = xgcd($z, $mod);
        die "Not pair-wise coprime!" unless 1 == $gcd;
        @y.push: $α;
    }

    return ([+] (^@eqns.elems).map: -> $i { @eqns[$i][0] * @y[$i] * @z[$i] }) % $M;
}


sub MAIN($file="15.in") {
    # Each disc gives a constraint:
    #    $t + initial + depth = 0        (mod positions)
    #    $t = -( initial + depth )       (mod positions)
    my @discs = gather for $file.IO.lines {
        if /^Disc <ws> "#"$<depth>=(\d+) .*? $<nr_pos>=(\d+) <ws> positions .*? at <ws> position <ws> $<initial>=(\d+)/ {
            take [ -( $<initial> + $<depth> ), +$<nr_pos> ];
        }
    }

    say solve_modulars(@discs);
}
