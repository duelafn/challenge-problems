#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ pairmap reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Dean::Util qw/ clprint /;
use lib '.';

use AoC ':all';
# file      # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix    # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)
# knot_hash # Day 10

# pairmap { $a, $b } $vm->registers;

sub to_binary { shift =~ s/(.)/sprintf("%04b", hex($1))/erg }

sub clear_region {
    my ($disk, @todo) = @_;
    while (@todo) {
        my ($x, $y) = @{shift @todo};
        next if $y < 0 or $y > $#{$disk};
        next if $x < 0 or $x > $#{$$disk[$y]};
        next unless $$disk[$y][$x];
        $$disk[$y][$x] = 0;

        push @todo, [$x-1, $y], [$x+1, $y], [$x, $y-1], [$x, $y+1];
    }
}


func MAIN() {
    my $key = "hfdlxzhv";

    is to_binary('a0c20170'), '10100000110000100000000101110000', "binary";

    my $count = 0;
    my @disk;
    for my $row (0..127) {
        my $hash = knot_hash("hfdlxzhv-$row");
        my $bin  = to_binary($hash);
        $count += ($bin =~ s/1/1/g);
        push @disk, [ split //, $bin ];
    }
    say "Count: $count";

    my $num_regions = 0;
    for my $y (0..$#disk) {
        for my $x (0..$#{$disk[$y]}) {
            if ($disk[$y][$x]) {
                $num_regions++;
                clear_region(\@disk, [$x,$y]);
            }
        }
    }
    say "Regions: $num_regions";
}


MAIN(@ARGV);
done_testing;

__DATA__
