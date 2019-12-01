#!/usr/bin/perl -w
use strict; use warnings; use 5.014;

use Dean::Util qw/ cat /;

local $_ = cat("01.in");
my $up = tr/(/(/;
my $dn = tr/)/)/;
say "Target floor: ", $up-$dn;

my ($floor, $n);
for (split //) {
    $n++;
    if (/\(/) { $floor++ }
    else { $floor-- }
    if ($floor < 0) {
        say "Entered the basement at step $n";
        exit;
    }
}
