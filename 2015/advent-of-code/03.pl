#!/usr/bin/perl -w
use strict; use warnings; use 5.014;

use Dean::Util qw/ cat /;

do {
    my ($x, $y) = (0, 0);
    my %house = ( "0,0" => 1 );
    my $unique = 1;
    for (cat($ARGV[0] || "03.in") =~ /([\^v<>])/g) {
        if    ($_ eq '^') { $y += 1 }
        elsif ($_ eq 'v') { $y -= 1 }
        elsif ($_ eq '<') { $x -= 1 }
        elsif ($_ eq '>') { $x += 1 }
        else { die "Found directive '$_'?" }
        $unique++ unless $house{"$x,$y"}++;
    }
    say "Part 1: $unique";
};

do {
    my @p = ([0, 0], [0, 0]);
    my %house = ( "0,0" => 1 );
    my $parity = 0;
    my $unique = 1;
    for (cat($ARGV[0] || "03.in") =~ /([\^v<>])/g) {
        my ($x, $y) = @{$p[$parity]};
        if    ($_ eq '^') { $y += 1 }
        elsif ($_ eq 'v') { $y -= 1 }
        elsif ($_ eq '<') { $x -= 1 }
        elsif ($_ eq '>') { $x += 1 }
        else { die "Found directive '$_'?" }
        $unique++ unless $house{"$x,$y"}++;
        @{$p[$parity]} = ($x, $y);
        $parity = $parity ? 0 : 1;
    }
    say "Part 2: $unique";
};
