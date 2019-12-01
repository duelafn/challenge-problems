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

sub load {
    my @fw;
    my @all;
    for (@_) {
        chomp;
        next unless $_;
        my ($a, $b) = split /:\s*/;
        $fw[$a] = $b;
        push @all, $b;
    }
    return \@fw;
}

sub show {
    my ($fw, $delay, $depth) = @_;
    print "T=", $delay+$depth;
    for my $p (0..$#{$fw}) {
        if ($$fw[$p]) {
            my $x = (($depth + $delay) % (2 * $$fw[$p] - 2));
            $x = (2 * $$fw[$p] - 2) - $x if $x >= $$fw[$p];
            print( ($p == $depth) ? "  ($x)" : "  [$x]" );
        } else {
            print( ($p == $depth) ? "  (.)"  : "  ..." );
        }
    }
    say "";
}

sub cost {
    my ($fw, $delay, $fast) = @_;
    my $cost = 0;
    for my $depth (0..$#{$fw}) {
        if ($$fw[$depth] and !(($depth + $delay) % (2 * $$fw[$depth] - 2))) {
            return 1 if $fast;# Note: cost may be zero, but we still got caught!
            $cost += $depth * $$fw[$depth];
        }
#         show($fw, $delay, $depth) if 7 == @$fw and ($delay == 10 or $delay == 4);
    }
    return $cost;
}

sub test { return cost(@_, 1) }

sub step {
    my ($start, $fw, $state) = @_;
    for my $p ($start..$#{$state}) {
        if ($$fw[$p]) {
            if ($$state[$p] == ($$fw[$p] - 1)) {
                $$state[$p] = 2 - $$fw[$p];
            } else {
                $$state[$p] += 1;
            }
        }
    }
}

sub cost_slow {
    my ($fw, $delay) = @_;
    my @state = (0)x@$fw;

    step(0, $fw, \@state) for 1..$delay;

    my $cost = 0;
    my $last = $#{$fw};
    for my $depth (0..$last) {
        $cost += $depth * $$fw[$depth] if $$fw[$depth] and 0 == $state[$depth];
        step($depth+1, $fw, \@state);
    }
    return $cost;
}

func MAIN() {
    my $fw = load(<DATA>);
    is cost($fw, 0), 24, "test";

    ok test($fw, $_) != 0, "test not safe: $_" for 1..9;
    is test($fw, 10), 0, "test safe";

    $fw = load(lines("13.in"));
    is cost($fw, 0), 1928, "step 1";

    my $offset = 1;
    $offset++ while test($fw, $offset);
    say "Safe at $offset";
    is $offset, 3830344, "part 2";
}


MAIN(@ARGV);
done_testing;

__DATA__
0: 3
1: 2
4: 4
6: 4
