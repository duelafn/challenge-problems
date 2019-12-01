#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ pairmap reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Dean::Util qw/ /;
use lib '.';

use AoC ':all';
# file      # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix    # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)
# knot_hash # Day 10

# pairmap { $a, $b } $vm->registers;

func move($x, $y, $dir) {
    if ($dir eq 'n')  { return ($x, $y+1) }
    if ($dir eq 's')  { return ($x, $y-1) }
    if ($dir eq 'ne') { return ($x+1, $y+1) }
    if ($dir eq 'nw') { return ($x-1, $y) }
    if ($dir eq 'se') { return ($x+1, $y) }
    if ($dir eq 'sw') { return ($x-1, $y-1) }
}

# http://keekerdc.com/2011/03/hexagon-grids-coordinate-systems-and-distance-calculations/
func hexnorm($x, $y) {
    return max(abs($x), abs($y), abs($x-$y))
}

sub distance {
    my ($x, $y) = (0, 0);
    my $maxdist = 0;
    for (split /,/, shift) {
        ($x, $y) = move($x, $y, $_);
        my $d = hexnorm($x, $y);
        $maxdist = $d if $d > $maxdist;
    }
    say "Max dist: $maxdist";
    return hexnorm($x, $y);
}


func MAIN() {
    is distance("ne,ne,ne"), 3, "ne,ne,ne";
    is distance("ne,ne,sw,sw"), 0, "ne,ne,sw,sw";
    is distance("ne,ne,s,s"), 2, "ne,ne,s,s";
    is distance("se,sw,se,sw,sw"), 3, "se,sw,se,sw,sw";

    for (lines("11.in")) {
        say "> ", distance($_);
    }
}


MAIN(@ARGV);
done_testing;

__DATA__
