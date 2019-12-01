#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Dean::Util qw/ BOLD /;
use lib '.';
use AoC ':all';

func MAIN() {
    my @off = do {
        no warnings 'qw';
        qw/ . 0,0 1,0 1,1 0,1 -1,1 -1,0 -1,-1 0,-1 1,-1 2,-1 2,0 2,1 2,2 /;
    };

    is join(",",offset($_)), $off[$_], "step $_" for 1..$#off;
    is sum(map abs($_), offset(1024)), 31, "step 1024";

    say "Part 1: ", sum(map abs($_), offset(325489));

    my $add = 5;
    my @grid = map [ (0)x(2*$add+1) ], 0..2*$add;

    my $val = 1;
    my $i = 1;
    my @test = qw/ 0 1 1 2 4 5 10 11 23 25 26 54 57 59 122 133 142 147 304 330 351 362 747 806 /;# https://oeis.org/A141481
    while ($val <= 325489) {
        if ($i <= $#test) {
            is $val, $test[$i], "step $i";
            unless ($val == $test[$i]) {
                show(\@grid);
                exit;
            }
        }

        my ($x, $y) = map $add + $_, offset($i);
        $grid[$y][$x] = $val;
        $i += 1;
        ($x, $y) = map $add + $_, offset($i);
        $val = $grid[$y-1][$x+1] + $grid[$y][$x+1] + $grid[$y+1][$x+1]
             + $grid[$y-1][$x]                     + $grid[$y+1][$x]
             + $grid[$y-1][$x-1] + $grid[$y][$x-1] + $grid[$y+1][$x-1]
             ;
    }
    show(\@grid);
    say "Part 2: $val";
}

func show($m) {
    say join " ", map sprintf("%7s", $_), @$_ for reverse @$m;
}

func offset($N) {
    my $n = $N;
    return (0,0) if 1 == $n;
    my ($i, $k) = (1, 1);
    while ($n > $k) {
        $n -= $k;
        $k = $i * $i;
        $i += 2;
        $k = $i * $i - $k;
    }

    # Right column:
    return (int($i/2), $n - int($i/2)) if $n < $i;

    # Top row:
    $n -= $i-1;
    return (int($i/2) - $n, int($i/2)) if $n < $i;

    # Left column:
    $n -= $i-1;
    return (-int($i/2), int($i/2) - $n) if $n < $i;

    # Bottom row:
    $n -= $i-1;
    return ($n - int($i/2), -int($i/2)) if $n < $i;

    die;
}



MAIN(@ARGV);
done_testing;

__DATA__
