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

sub RAM {
    my $N = shift;
    my @ram;
    my @len = (split //, shift @_);
    push @ram, [ ('.')x$N, (map ".", @len), ('.')x$N ] for 1..$N;
    push @ram, [ ('.')x$N, @len, ('.')x$N ];
    push @ram, [ ('.')x$N, (split //), ('.')x$N ] for @_;
    push @ram, [ ('.')x$N, (map ".", @len), ('.')x$N ] for 1..$N;
    return \@ram;
}

sub show {
    my ($pos, $vec, $ram) = @_;
    say "At @$pos;  Facing @$vec";
    say @$_ for @$ram;
    say "";
}

sub step_virus1 {
    my ($pos, $vec, $ram) = @_;
    my $rv = 0;
    die "Off the map\n" unless $$ram[$$pos[1]] and $$ram[$$pos[1]][$$pos[0]];

    # Turn
    if ($$ram[$$pos[1]][$$pos[0]] eq '#') {
        @$vec = (-$$vec[1], $$vec[0]); # Turn right
    } else {
        @$vec = ($$vec[1], -$$vec[0]); # Turn left
        $rv = 1; # Will infect
    }

    # Infect
    $$ram[$$pos[1]][$$pos[0]] =~ tr/#./.#/;

    # Move
    $$pos[0] += $$vec[0];
    $$pos[1] += $$vec[1];

    return $rv;
}

sub step_virus2 {
    my ($pos, $vec, $ram) = @_;
    my $rv = 0;
    die "Off the map\n" unless $$ram[$$pos[1]] and $$ram[$$pos[1]][$$pos[0]];

    # Turn
    for ($$ram[$$pos[1]][$$pos[0]]) {
        if (/\./) {
            @$vec = ($$vec[1], -$$vec[0]); # Turn left
        } elsif (/W/) {
            $rv = 1; # Will infect, do not turn
        } elsif (/#/) {
            @$vec = (-$$vec[1], $$vec[0]); # Turn right
        } elsif (/F/) {
            @$vec = (-$$vec[0], -$$vec[1]); # About-face
        }
    }

    # Infect
    $$ram[$$pos[1]][$$pos[0]] =~ tr/.W#F/W#F./;

    # Move
    $$pos[0] += $$vec[0];
    $$pos[1] += $$vec[1];

    return $rv;
}


func MAIN() {
    {   chomp(my @lines = <DATA>);
        my $ram = RAM(1000, @lines);
        my $pos = [ int(@$ram/2), int(@$ram/2) ];
        my $vec = [ 0, -1 ];

        my $infect = 0;
        $infect += step_virus1($pos, $vec, $ram) for 1..10_000;
        is $infect, 5587, "test";
    }

    {   my @lines = lines("22.in");
        my $ram = RAM(1000, @lines);
        my $pos = [ int(@$ram/2), int(@$ram/2) ];
        my $vec = [ 0, -1 ];

        my $infect = 0;
        $infect += step_virus1($pos, $vec, $ram) for 1..10_000;
        is $infect, 5339, "part 1";
    }

    {   my @lines = lines("22.in");
        my $ram = RAM(1000, @lines);
        my $pos = [ int(@$ram/2), int(@$ram/2) ];
        my $vec = [ 0, -1 ];

        my $infect = 0;
        $infect += step_virus2($pos, $vec, $ram) for 1..10_000_000;
        is $infect, 2512380, "part 2";
    }
}


MAIN(@ARGV);
done_testing;

__DATA__
..#
#..
...
