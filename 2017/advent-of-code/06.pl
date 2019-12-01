#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Dean::Util qw/ /;
use lib '.';
use sort 'stable';

use AoC ':all';
# file   # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)

our @BLOCK = qw/ 10    3   15  10  5   15  5   15  9   2   5   8   5   2   3   6 /;
# our @BLOCK = qw/ 0 2 7 0 /;

func MAIN() {
    my %seen = ( "@BLOCK" => 0 );
    my $i = 0;
    while (1) {
        $i++;
        my @idx = sort { $BLOCK[$b] <=> $BLOCK[$a] } 0..$#BLOCK;
        my $alloc = $BLOCK[$idx[0]];
        $BLOCK[$idx[0]] = 0;
        my $give = ($idx[0] + 1) % @BLOCK;
        while ($alloc--) {
            $BLOCK[$give]++;
            $give = ($give + 1) % @BLOCK;
        }
        if (exists($seen{"@BLOCK"})) {
            say qq|@BLOCK: $seen{"@BLOCK"} .. $i (@{[ $i - $seen{"@BLOCK"} ]})|;
            exit;
        } else {
            $seen{"@BLOCK"} = $i;
        }
    }
}



MAIN(@ARGV);
done_testing;

__DATA__
