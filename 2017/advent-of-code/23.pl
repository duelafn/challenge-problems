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


func step_vm($lines, $vm) {
    my $lineno = $vm->lineno;
    if ($lineno < 0 or $lineno > $#{$lines}) {
        say $vm->reg->{_name}, " terminated" unless $vm->reg->{_terminated};
        $vm->reg->{_terminated} = 1;
        return undef;
    }
    my $cmd = $$lines[$lineno];
    my $h = $vm->val("h");
    say "$lineno: ", join " ", map "$_=".$vm->val($_), "a".."h";
    if (2 == @$cmd) {
        $vm->uniop(@$cmd);
    } else {
        $vm->uniop(inc => "_mul") if $$cmd[0] eq 'mul';
        $vm->binop(@$cmd);
    }
    say "h: $h -> ", $vm->val("h") if $h != $vm->val("h");
    return $vm->lineno;
}



func MAIN() {
    my $e = 8399;
    my $vm = VM->new( lineno => 17, reg => { _name => 0, a => 1, b => 8400, c => 25400, d => 4, f => 1, e => $e, g => (2*$e-2), h => 0 } );

#     chomp(my @lines = <DATA>);
    my @lines = lines("23.in");

    @lines = map [ split ], grep /\S/, @lines;
    my $i = 0;
    while (1) {
        $i++;
        last unless defined(step_vm(\@lines, $vm));
    }
    say $vm->val("_mul");

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
# done_testing;

__DATA__

set b 84
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23

VERSION 1
---------
b = 84
c = b
goto AA if a == 0;
b = 100 * b + 100000
c = b + 17000
AA:
f = 1
d = 2
CC:
e = 2
BB:
g = d * e - b
f = 0 if g == 0
e -= 1
g = e - b
goto BB if g != 0
d -= 1
g = d - b
goto CC if g != 0
h -= 1 if f == 0
g = b - c
exit if g == 0
b += 17
goto AA


VERSION 2
---------
b = 84
c = b
if (a) {
    b = 100 * b + 100_000
    c = b + 17000
}

while (1) {
    f = 1
    d = 2

    do {
        e = 2
        do {
            g = d * e - b
            f = 0 if g == 0
            e += 1
            g = e - b
        } while g != 0
        d += 1
        g = d - b
    } while g != 0

    # h = number of times f is zero
    # Yes, h is number of primes in:
    # 108_400, 108_417, ..., 125_400
    h += 1 if f == 0
    g = b - c
    exit if g == 0
    b += 17
}

# PERL6:
my @x = 108_400, 108_417 ... 125_400;
@x.elems - @x.grep({ .is-prime }).elems;
