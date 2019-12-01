#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ pairmap reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Dean::Util qw/ /;
use Time::HiRes qw/ time /;
use lib '.';

use AoC ':all';
# file      # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix    # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)
# knot_hash # Day 10

# pairmap { $a, $b } $vm->registers;

func spin($r, $n)         { unshift @$r, splice @$r, -$n, $n; }
func exchange($r, $a, $b) { @$r[$a, $b] = @$r[$b, $a] }
func partner($r, $a, $b)  { @$r = (split /\s+/, ((("@$r" =~ s/$a/x/r) =~ s/$b/$a/r) =~ s/x/$b/r)) }

sub partA {
    my $r = shift;
    my $orig = "@$r";
    my $N = shift;
    my @act = map { split /,/ } @_;
    while ($N--) {
        for (@act) {
            if    (/^s(\d+)/) { spin($r, $1) }
            elsif (/^x(\d+)\/(\d+)/) { exchange($r, $1, $2) }
            elsif (/^p(\w+)\/(\w+)/) { partner($r, $1, $2) }
        }
    }
    return join "", @$r;
}

sub find_order {
    my $r = shift;
    my $orig = "@$r";
    my @act = map { split /,/ } @_;
    my $order = 0;
    while (1) {
        for (@act) {
            if    (/^s(\d+)/) { spin($r, $1) }
            elsif (/^x(\d+)\/(\d+)/) { exchange($r, $1, $2) }
            elsif (/^p(\w+)\/(\w+)/) { partner($r, $1, $2) }
        }
        $order++;
        return $order if "@$r" eq $orig;
    }
}

sub get_perm { [ map ord($_)-97, split //, shift ] }

# Guess 1: jdkhongimbalpefc  landau
# Guess 3: dlkpgmjciehnobfa  landau (total of 1 billion <-> repeat 999999999?)
# Final:   pnhajoekigcbflmd  order is actually 60  (not a divisor of 140)

sub partB {
    my $n = shift;
    my $r = [ ("a".."z")[0..($n-1)] ];
    my $total_runs = shift;
    my @act = map { split /,/ } @_;
    my $order = find_order([ @$r ], @act);

    my $repeat = ($total_runs % $order) - 1;
    say "Repeats: $repeat <- ($total_runs % $order)" if $total_runs >= $order or $repeat < 0;
    return join "", @$r if $repeat < 0;

    my @part;
    for (@act) {
        if    (/^s(\d+)/) { spin($r, $1) }
        elsif (/^x(\d+)\/(\d+)/) { exchange($r, $1, $2) }
        elsif (/^p(\w+)\/(\w+)/) { push @part, "$1$2" }
    }

    my $perm = get_perm(join "", @$r);
    die "@$r ne @{[ ('a'..'z')[@$perm] ]}" unless "@$r" eq "@{[ ('a'..'z')[@$perm] ]}";
    partner($r, split //) for @part;

    while ($repeat--) {
        @$r = @$r[@$perm];
        partner($r, split //) for @part;
    }
    return join "", @$r;
}

func MAIN() {
    my @dance = lines("16.in");

    is partB(5, 1, "s1"), "eabcd", "T1";
    is partB(5, 1, "s1,x3/4"), "eabdc", "T2";
    is partB(5, 1, "s1,x3/4,pe/b"), "baedc", "T3";
    my $partA = partB(16, 1, @dance);
    is $partA, "ceijbfoamgkdnlph", "Part A";
    say $partA;

    is partA(["a".."e"], 2, "s1,x3/4,pe/b"), "ceadb", "small, 2 total, method A";

    is partB(5, 2, "s1,x3/4,pe/b"), "ceadb", "small, 2 total";
    is partB(5, 3, "s1,x3/4,pe/b"), partA(["a".."e"], 3, "s1,x3/4,pe/b"), "small, 3 total";

    is partB(16, 2, @dance), partA(["a".."p"], 2, @dance), "big, 2 total";
    is partB(16, 3, @dance), partA(["a".."p"], 3, @dance), "big, 3 total";
    is partB(16, 4, @dance), partA(["a".."p"], 4, @dance), "big, 4 total";

    for my $n (sort { $a <=> $b } qw/ 139 140 141 200 280 500 503 /) {
        is partB(16, $n, @dance), partA(["a".."p"], $n, @dance), "big, $n total";
    }

    my $partB = partB(16, 1000000000, lines("16.in"));
    say $partB;
}

sub partB_old {
    my $n = shift;
    my $pos = [ 0..$n-1 ];
    my $ltr = [ 0..$n-1 ];
    my $repeat = shift;
    my @act = map { split /,/ } @_;
    for (@act) {
        if    (/^s(\d+)/) { spin($pos, $1) }
        elsif (/^x(\d+)\/(\d+)/) { exchange($pos, $1, $2) }
        elsif (/^p(\w+)\/(\w+)/) { partner($pos, $1, $2) }
    }

    if ($repeat > 0) {
        my @pos_perm = @$pos;
        my @ltr_perm = @$ltr;
        while ($repeat--) {
            @$pos = @$pos[@pos_perm];
            @$ltr = @$ltr[@ltr_perm];
        }
    }

    my @A = ("a".."z");
    return join "", @A[@$ltr[@$pos]];
}

func MAIN_slow() {
    is partA(["a".."e"], 1, "s1"), "eabcd", "T1";
    is partA(["a".."e"], 1, "s1,x3/4"), "eabdc", "T2";
    is partA(["a".."e"], 1, "s1,x3/4,pe/b"), "baedc", "T3";
    my $partA = partA([ "a".."p" ], 1, lines("16.in"));
    say $partA;

    is partB(["a".."e"], get_perm("baedc"), 1), "baedc", "T4";
    is partA([split //, "baedc"], 1, "s1,x3/4,pe/b"), "ceadb", "T5";

    is partA(["a".."e"], 2, "s1,x3/4,pe/b"), "ceadb", "T6";
    my $partB = partA([ "a".."p" ], 1000000001, lines("16.in"));
    say $partB;
}


MAIN(@ARGV);
done_testing;

__DATA__
