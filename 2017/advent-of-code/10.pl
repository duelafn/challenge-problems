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
# file   # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)

# pairmap { $a, $b } $vm->registers;


sub step {
    my ($l, $p, $len, $skip) = @_;
    if ($p + $len >= @$l) {
        my $n = @$l;
        my @tmp = (@$l, @$l);
        my @new = reverse splice @tmp, $p, $len;
        splice @tmp, $p, 0, @new;
        @$l = (@tmp[$n..($len+$p-1)], @tmp[($len+$p-$n)..($n-1)]);
    } else {
        my @new = reverse splice @$l, $p, $len;
        splice @$l, $p, 0, @new;
    }

    return( ($p + $len + $skip) % @$l );
}

sub hash1 {
    my ($n, $len, $pos, $skip, $l) = @_;
    $pos //= 0;
    $skip //= 0;
    $l //= [ 0..($n-1) ];
    for (@$len) {
        $pos = step($l, $pos, $_, $skip);
        $skip += 1;
    }
    return $$l[0] * $$l[1] unless wantarray;
    return ($pos, $skip);
}


# Moved to AoC.pm
#----------------
sub hash_reduce {
    my @dense;
    while (@_) {
        push @dense, sprintf "%02x", reduce { $a ^ $b } splice @_, 0, 16;
    }
    return join "", @dense;
}

sub hash {
    my @magic = (17, 31, 73, 47, 23);
    my $str = shift;
    my @key = (map(ord($_), split //, $str), @magic);

    my $n = 256;
    my ($p, $skip, $l) = (0, 0, [0..255]);
    for (1..64) {
        for my $len (@key) {
            if ($p + $len >= @$l) {
                my @tmp = (@$l, @$l);
                my @new = reverse splice @tmp, $p, $len;
                splice @tmp, $p, 0, @new;
                @$l = (@tmp[$n..($len+$p-1)], @tmp[($len+$p-$n)..($n-1)]);
            } else {
                my @new = reverse splice @$l, $p, $len;
                splice @$l, $p, 0, @new;
            }
            $p = ($p + $len + $skip) % $n;
            $skip += 1;
        }
    }
    return hash_reduce(@$l);
}
#----------------



func MAIN() {
    is scalar hash1(5, [3, 4, 1, 5]), 12, "test";
    is scalar hash1(256, [197,97,204,108,1,29,5,71,0,50,2,255,248,78,254,63]), 40132, "PART 1";

    is knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272", "test: empty";
    is knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd", "test: AoC 2017";
    is knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d", "test: 1,2,3";
    is knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e", "test: 1,2,4";

#     say "Part 2: ", knot_hash("197,97,204,108,1,29,5,71,0,50,2,255,248,78,254,63");
    is knot_hash("197,97,204,108,1,29,5,71,0,50,2,255,248,78,254,63"), "35b028fe2c958793f7d5a61d07a008c8", "PART 2";
}



MAIN(@ARGV);
done_testing;

__DATA__
