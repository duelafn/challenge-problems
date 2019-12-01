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


sub step {
    my ($s, %opt) = @_;
    my $close = $$s{particles}[0];
    my %seen;
    for my $p (@{$$s{particles}}) {
        $$p{d} = 0;
        for (0, 1, 2) {
            $$p{v}[$_] += $$p{a}[$_];
            $$p{p}[$_] += $$p{v}[$_];
            $$p{d} += abs($$p{p}[$_]);
        }
        $seen{"@{$$p{p}}"}++;
        $close = $p if $$p{d} < $$close{d};
    }
#     use Data::Dump 'pp'; say pp \%seen; exit;
    @{$$s{particles}} = grep { $seen{"@{$$_{p}}"} < 2 } @{$$s{particles}} if $opt{collide};
    $$s{close} = $close;
}

sub load {
    my %s;
    my $n = 0;
    for (@_) {
        next unless /\S/;
        my %p = ( n => $n++ );
        die "position error: $_" unless /p=<([-\d,]+)>/;
        $p{p} = [ split /,/, $1 ];
        die "velocity error: $_" unless /v=<([-\d,]+)>/;
        $p{v} = [ split /,/, $1 ];
        die "acceleration error: $_" unless /a=<([-\d,]+)>/;
        $p{a} = [ split /,/, $1 ];
        push @{$s{particles}}, \%p;
    }
    return \%s;
}


func MAIN() {
#     my $s = load(<DATA>);
    my $s = load(lines("20.in"));
    my $i = 0;
    my $close = $$s{particles}[0];
    while (1) {
        $i++;
        step($s, collide => 1);
        if ($$close{n} <=> $$s{close}{n} or 0 == $i % 10000) {
            $close = $$s{close};
#             use Data::Dump 'pp'; say pp $s;
            printf "% 10d: particle %d at <%d, %d, %d> is distance %d  (%d particles remain)\n",
                $i, $$close{n}, @{$$close{p}}, $$close{d}, 0+@{$$s{particles}};
        }
    }
}



MAIN(@ARGV);
# done_testing;

# p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
# p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>

__DATA__
p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>
