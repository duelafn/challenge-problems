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


func step_vm($lines, $vm, $othervm, $i) {
    my $lineno = $vm->lineno;
    if ($lineno < 0 or $lineno > $#{$lines}) {
        say $vm->reg->{_name}, " terminated" unless $vm->reg->{_terminated};
        $vm->reg->{_terminated} = 1;
        return undef;
    }
    my $cmd = $$lines[$lineno];
    if ($$cmd[0] =~ /^(?:snd|rcv)/) {
        $vm->binop(@$cmd, $othervm);
        return -1 if $lineno == $vm->lineno;
    } elsif (2 == @$cmd) {
        $vm->uniop(@$cmd);
    } else {
        $vm->binop(@$cmd);
    }
    return $vm->lineno;
}


func MAIN() {
    my $vm0 = VM->new( reg => { _name => 0, p => 0 } );
    my $vm1 = VM->new( reg => { _name => 1, p => 1 } );

#     chomp(my @lines = <DATA>);
    my @lines = lines("18.in");

    @lines = map [ split ], grep /\S/, @lines;
    my $i = 0;
    while (1) {
        $i++;
        my $a = step_vm(\@lines, $vm0, $vm1, $i);
        my $b = step_vm(\@lines, $vm1, $vm0, $i);
        if (($a < 0 or !defined($a)) and ($b < 0 or !defined($b))) {
            $vm0->dump_registers;
            $vm1->dump_registers;
            say "VM0 send count: ", $vm0->reg->{_snd_count};
            say "VM1 send count: ", $vm1->reg->{_snd_count};
            exit;
        }
    }
}



MAIN(@ARGV);
# done_testing;

# set a 1
# add a 2
# mul a a
# mod a 5
# snd a
# set a 0
# rcv a
# jgz a -1
# set a 1
# jgz a -2


__DATA__
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
