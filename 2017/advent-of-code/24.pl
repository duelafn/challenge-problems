#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ pairmap reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Data::Dump 'pp';
use Dean::Util qw/ /;
use lib '.';

use AoC ':all';
# file      # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix    # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)
# knot_hash # Day 10

# pairmap { $a, $b } $vm->registers;

sub best_strength {
    my $strength = sum( map split(/:/), @{$_[0]} );
    return $strength;
}

sub best_length {
    my $length = @{$_[0]};
    my $strength = sum( map split(/:/), @{$_[0]} );
    return sprintf "%d%05d", $length, $strength;
}

sub dfs {
    my ($bridge, $pieces, $strength) = @_;
    my ($last) = reverse split /\D/, $$bridge[-1];
    my @next = grep /^$last:/, keys %$pieces;
    my $best = $strength->($bridge);
    for my $a (@next) {
        my %p = %$pieces;
        $b = ($a =~ s/(\d+):(\d+)/$2:$1/r);
        for ($a, $b) { $p{$_}--; delete $p{$_} unless $p{$_} }
        my $score = dfs([ @$bridge, $a ], \%p, $strength);
        $best = $score if $score > $best;
    }
    return $best;
}

sub load {
    my %p;
    for (@_) {
        chomp;
        my ($a, $b) = split /\D/;
        $p{"$a:$b"}++;
        $p{"$b:$a"}++;
    }
    return \%p;
}


func MAIN() {
    my @data = <DATA>;
    is dfs(["0:0"], load(@data), \&best_strength), 31, "test 1";
    is dfs(["0:0"], load(lines("24.in")), \&best_strength), 1656, "part 1";

    is 0+substr(dfs(["0:0"], load(@data), \&best_length), -5), 19, "test 2";
    is 0+substr(dfs(["0:0"], load(lines("24.in")), \&best_length), -5), 1642, "part 2";
}

MAIN(@ARGV);
done_testing;

__DATA__
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
