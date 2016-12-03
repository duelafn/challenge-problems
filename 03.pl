#!/usr/bin/env perl6
use v6.c;

multi MAIN('rows') {
    my $possible = 0;
    for "03.in".IO.lines -> $line {
        my @num = sort $line.comb(/\d+/)».Int;
        $possible++ if @num[0] + @num[1] > @num[2];
    }
    say $possible;
}

multi MAIN('golf') {
    say "rows: ", [+] "03.in".IO.lines.map:{my \n=.comb(/\d+/)».Int.sort;n[0]+n[1]>n[2]};
    say "cols: ", [+] flat([Z] "03.in".IO.lines».comb(/\d+/)».Int).rotor(3)».sort.map:{.[0]+.[1]>.[2]};
}

multi MAIN('cols') {
    my $possible = 0;
    for flat([Z] "03.in".IO.lines».comb(/\d+/)».Int) -> $a, $b, $c {
        my @num = sort($a, $b, $c);
        $possible++ if @num[0] + @num[1] > @num[2];
    }
    say $possible;
}
