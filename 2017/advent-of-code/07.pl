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
# file   # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)

use Data::Dump 'pp';


func MAIN() {
    my %tree;
    for (lines("07.in")) {
#     for (<DATA>) {
        chomp;
        my ($self, $weight, @child) = split /[ \(\)\-\>,]+/;
        next unless $self;
        $tree{$self}{weight} = $weight;
        $tree{$self}{child} = \@child;
        $tree{$_}{parent} = $self for @child;
    }
    my ($a) = keys %tree;
    $a = $tree{$a}{parent} while $tree{$a}{parent};
    say $a;
    check_weight(\%tree, $a);
}

func check_weight($tree, $node) {
    my $weight = $$tree{$node}{weight};

    return $weight unless $$tree{$node}{child};

    my $error;
    my %cweight;
    my @chld_weight;
    for my $chld (@{$$tree{$node}{child}}) {
        $cweight{$chld} = check_weight($tree, $chld);
        push @chld_weight, $cweight{$chld};
        $weight += $cweight{$chld};
        $error = 1 if $chld_weight[0] != $cweight{$chld};
    }

    if ($error) {
        say "ERROR at $node";
        @chld_weight = sort { $a <=> $b } @chld_weight;
        my $expected = $chld_weight[@chld_weight/2];
        for my $c (keys %cweight) {
            $cweight{$c} = {
                self => $$tree{$c}{weight},
                children => sum( map check_weight($tree, $_), @{$$tree{$c}{child} || []} ),
                total => $cweight{$c},
                should_be => ($$tree{$c}{weight} - ($cweight{$c} - $expected)),
            };
            $cweight{$c}{should_be} = "*$cweight{$c}{should_be}*" if $cweight{$c}{should_be} != $cweight{$c}{self};
        }
        say pp \%cweight;
        exit;
    }

    return $weight;
}

#     my @chunk = split /\n\n+/, do { local $/; scalar <DATA> };

#     while (my $m = read_matrix(\*DATA)) {
#     }



MAIN(@ARGV);
done_testing;

__DATA__

pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
