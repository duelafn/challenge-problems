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

sub net_size {
    my ($tree, $start) = @_;
    my @todo = ($start);
    my %seen;
    my $n = 0;
    while (@todo) {
        my $x = shift @todo;
        unless ($seen{$x}++) {
            $n++;
            push @todo, @{$$tree{$x}};
        }
    }
    return $n unless wantarray;
    return ($n, [ keys %seen ]);
}

sub build {
    my $tree = {};
    for (@_) {
        chomp;
        my ($a, $b) = split /\s*\<\-\>\s*/;
        push @{$$tree{$a}}, split /\s*,\s*/, $b;
    }
    return $tree;
}

func MAIN() {
    my $tree = build(<DATA>);
    is scalar net_size($tree, 0), 6, "test";

    $tree = build(lines("12.in"));
    my $groups = 0;
    my $start  = 0;
    while (%$tree) {
        $groups++;
        my ($n, $seen) = net_size($tree, $start);
        say "Start $start: size $n";
        delete @$tree{@$seen};
        ($start) = sort { $a <=> $b } keys %$tree;
    }
    say "$groups groups";
}


MAIN(@ARGV);
done_testing;

__DATA__
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
