#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Dean::Util qw/ /;
use lib '.';

use AoC ':all';
# file   # lines($file) -> @chomped_lines, readfile($file) -> $slurp
# matrix # read_matrix(\*DATA) OR read_matrix($file)  -> \@M   ::opts::  sep => qr/\s+/, comments => 1 (ignore /^#/), blank => 1 (ignore /^\s*$/), one => 1 (return after /^\s*$/)


func MAIN() {
    my @code = grep length($_), lines("05.in");
    my $i = 0;
    my $step = 0;
    while ($i >=0 and $i < @code) {
        $step++;
        my $d = $code[$i];
        if ($code[$i] >= 3) { $code[$i] -= 1 }
        else { $code[$i] += 1 }
        $i += $d;
    }
    say $step;

}


MAIN(@ARGV);
done_testing;

__DATA__
