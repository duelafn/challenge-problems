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

sub part1 {
    my ($step, $last) = @_;
    $last //= 2017;
    my $pos = 0;
    my @buf = (0);
    for my $i (1..$last) {
        $pos = 1 + (($pos + $step) % @buf);
        splice @buf, $pos, 0, $i;
#         say "@buf" if $i < 10 and $step < 10;
    }
    return $buf[($pos + 1) % @buf];
}

sub part2_dumb {
    my ($step, $last) = @_;
    $last //= 2017;
    my $pos = 0;
    my @buf = (0);
    for my $i (1..$last) {
        $pos = 1 + (($pos + $step) % @buf);
        splice @buf, $pos, 0, $i;
#         say "@buf" if $i < 10 and $step < 10;
    }
    my $i = 0;
    $i = ($i + 1) % @buf while $buf[$i];
    return $buf[($i + 1) % @buf];
}

sub part2 {
    my ($step, $last) = @_;
    $last //= 50_000_000;
    my $pos0 = 0;
    my $pos = 0;
    my $next = 0;
    my $i = 0;
    while ($i < $last) {
        $i++;
        $pos = 1 + (($pos + $step) % $i);
        if ($pos < $pos0) {
            $pos0++;
        } elsif (($pos - 1) == $pos0) {
            $next = $i;
        }
    }
    return $next;
}

func MAIN() {
    is part1(3), 638, "Test";
    is part1(343), 1914, "Part A";

    is part2(3, 2017), part2_dumb(3), "Test 2";
    is part2(3, 9), part2_dumb(3, 9), "Test 2";

    say part2(343, 50_000_000); # 6.259s
}


MAIN(@ARGV);
done_testing;

__DATA__
