#!/usr/bin/perl -w
use strict; use warnings; use 5.014;

use Dean::Util qw/ cat /;

my $paper = 0;
my $ribbon = 0;
for (cat("02.in")) {
    my ($a, $b, $c) = sort { $a <=> $b } /(\d+)/g;
    $paper += 2*($a*$b+$a*$c+$b*$c) + $a*$b;
    $ribbon += 2*($a+$b) + $a*$b*$c;
}
say "Part 1: $paper";
say "Part 2: $ribbon";
