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

sub maze {
    my @maze;
    push @maze, [ split // ] for @_;
    return \@maze;
}

func get($maze, $x, $y) {
    if ($y >= 0 and $y < @$maze) {
        if ($x >= 0 and $x < @{$$maze[$y]}) {
            return $$maze[$y][$x];
        }
    }
    return "";
}

sub solve_maze {
    my ($maze) = @_;
    my ($dx, $dy) = (0, 1);
    my ($x, $y) = (0, 0);
    ($x) = grep $$maze[0][$_] eq '|', 0..$#{$$maze[0]};

    my @chk;
    my $step = 0;
    while (1) {
        $step++;
#     for (1..20) {
        my $here = get($maze, $x, $y);
#         say "($x, $y): $here";
        if ($here =~ /\w/) {
            push @chk, $here;
#             say "Found $here";
        }

        unless (get($maze, $x+$dx, $y+$dy) =~ /\S/) {
            # Left:
            if (get($maze, $x-$dy, $y+$dx) =~ /\S/) {
                ($dx, $dy) = (-$dy, $dx);
            }
            # Right:
            elsif (get($maze, $x+$dy, $y-$dx) =~ /\S/) {
                ($dx, $dy) = ($dy, -$dx);
            }
            else { last }
        }
        $x += $dx; $y += $dy;
    }
    say "$step Steps";
    return join "", @chk;
}


func MAIN() {
    my $maze;

    $maze = maze(<DATA>);
    is solve_maze($maze), "ABCDEF", "Test";

    $maze = maze(lines("19.in"));
    say solve_maze($maze);

}

#     my $vm = VM->new;
# #     for (<DATA>) {
# #         chomp;
#     for (lines("08.in")) {
#         next unless $_;
#         die "Parse error in '$_'" unless /
#             (?<r1>$re_name)
#         \s+ (?<op>$re_binop)
#         \s+ (?<val>$re_int|$re_name)
#         \s+ if
#         \s+ (?<c1>$re_int|$re_name)
#         \s+ (?<test>$re_cmp)
#         \s+ (?<c2>$re_int|$re_name)
#         /x;
#         my %x = %+;
#     }


#     my @chunk = split /\n\n+/, do { local $/; scalar <DATA> };


#     while (my $m = read_matrix(\*DATA)) {
#     }



MAIN(@ARGV);
done_testing;

__DATA__
     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
