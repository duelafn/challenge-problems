#!/usr/bin/env perl6
use v6.c;

my %SORT = (
    most  => { $^b.value <=> $^a.value || $^a.key cmp $^b.key },
    least => { $^a.value <=> $^b.value || $^a.key cmp $^b.key },
);

sub MAIN($file="06.in", :$method where { %SORT{$_}:exists } = 'most') {
    my @hist;
    for $file.IO.lines -> $line {
        for $line.comb.kv -> $idx, $char {
            (@hist[$idx] //= BagHash.new){$char}++;
        }
    }
    put join "", @hist.map: { .pairs.sort(%SORT{$method}).first.key };
}
