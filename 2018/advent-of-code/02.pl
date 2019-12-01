#!/usr/bin/env perl6
use v6.c;

sub checksum($s) {
    my @rv = (0, 0);
    for bag($s.comb).invert -> $kv {
        @rv[0] = 1 if $kv.key == 2;
        @rv[1] = 1 if $kv.key == 3;
    }
    return @rv;
}

sub levenshtein($a, $b) {
    [+] $a.comb >>ne<< $b.comb
}


sub MAIN(IO() $file="02.in") {
    my @sum = (0, 0);
    for $file.lines -> $key {
        @sum >>+=<< checksum($key);
    }
    put "Checksum ({@sum}): ", [*] @sum;

    for $file.lines.combinations(2) -> [$a, $b] {
        if 1 == levenshtein($a, $b) {
            put $a;
            put $b;
            put (($a.comb >>eq<< $b.comb) Z $a.comb).grep({ $_[0] }).map({ $_[1] }).join("");
        }
    }
}
