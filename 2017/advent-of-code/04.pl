#!/usr/bin/perl -w
use strict; use warnings; use 5.024;
use Time::Piece;
use Test::More;
use Method::Signatures::Simple;
use Path::Class; # $file->slurp(chomp => 1, iomode => ':crlf:encoding(UTF-8)'); $dir->children(no_hidden => 1)
use List::Util qw/ reduce any all none first max min sum product shuffle /;# reduce {$a+$b} 0, @list;
use Dean::Util qw/ cat /;
use lib '.';
use AoC ':all';

func MAIN() {
    my $valid = 0;
  pass:
    for (cat "04.in") {
        chomp;
        my %seen;
        for (split) {
            my $key = join "", sort(split //);
            next pass if $seen{$key}++;
        }
        $valid++;
    }
    say $valid;
}


MAIN(@ARGV);
done_testing;

__DATA__
