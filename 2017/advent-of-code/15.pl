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


sub gen {
    my ($current, $mul, $mod) = @_;
    $mod //= 2147483647;
    return sub { sprintf "%032b", ($current = ($current * $mul) % $mod) }
}

sub genB {
    my ($current, $mul, $criteria, $mod) = @_;
    $mod //= 2147483647;
    return sub {
        do {
            $current = ($current * $mul) % $mod
        } while ($current % $criteria);
        sprintf "%032b", $current;
    }
}

sub bin { sprintf "%032b", shift }

# Generator A starts with 883
# Generator B starts with 879

sub test {
    my $A = gen(65, 16807);
    my $B = gen(8921, 48271);

    my $count = 40_000_000;
    my $total = 0;
    my $s = 32 - 16;
    while ($count--) {
        $total++ if substr($A->(), $s) eq substr($B->(), $s);
    }
    is $total, 588, "Test 1";
}

sub partA {
    my $A = gen(883, 16807);
    my $B = gen(879, 48271);

    my $count = 40_000_000;
    my $total = 0;
    my $s = 32 - 16;
    while ($count--) {
        $total++ if substr($A->(), $s) eq substr($B->(), $s);
    }
    say $total;
    is $total, 609, "Part A";
}

sub partB {
    my $A = genB(883, 16807, 4);
    my $B = genB(879, 48271, 8);

    my $count = 5_000_000;
    my $total = 0;
    my $s = 32 - 16;
    while ($count--) {
        $total++ if substr($A->(), $s) eq substr($B->(), $s);
    }
    say $total;
    is $total, 253, "Part B";
}

func MAIN() {
    test();
    partA();
    partB();
}


MAIN(@ARGV);
done_testing;

__DATA__
