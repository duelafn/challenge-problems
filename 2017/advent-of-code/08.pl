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

package VM1 {
    use Moose;
    use MooseX::StrictConstructor;
    use Method::Signatures::Simple;
    extends 'VM';
    use AoC ':all';

    method run($cmd) {
        die "Parse error in '$_'" unless $cmd =~ /
            (?<r1>$re_name)
        \s+ (?<op>$re_binop)
        \s+ (?<val>$re_int|$re_name)
        \s+ if
        \s+ (?<c1>$re_int|$re_name)
        \s+ (?<test>$re_cmp)
        \s+ (?<c2>$re_int|$re_name)
        /x;
        my %x = %+;

        if ($self->test(@x{qw/ test c1 c2 /})) {
            $self->binop(@x{qw/ op r1 val /});
        }
    }

    no Moose;
    __PACKAGE__->meta->make_immutable;
}



func MAIN() {
    my $vm = VM1->new;

    my $max = 0;
#     for (<DATA>) {
#         chomp;
    for my $cmd (lines("08.in")) {
        next unless $cmd;
        $vm->run($cmd);

        my $mm = max($vm->register_values);
        $max = $mm if $mm > $max;
    }

    say "max: ", max($vm->register_values);
    say "max-max: ", $max;
}
# max: 4163
# max-max: 5347


func MAIN1() {
    my %reg;
#     for my $line (<DATA>) {
#         chomp($line);
    my $max = 0;
    for my $line (lines("08.in")) {
        next unless $line;

        my ($reg1, $act, $val1, $if, $reg2, $op, $val2) = split /\s+/, $line;
        if (eval "".($reg{$reg2}||0)." $op $val2") {
            if ($act eq 'inc') {
                $reg{$reg1} += $val1;
            } elsif ($act eq 'dec') {
                $reg{$reg1} -= $val1;
            }
        }
        my $mm = max(values %reg);
        $max = $mm if $mm > $max;
    }
    say "max: ", max(values %reg);
    say "max-max: ", $max;
}



MAIN(@ARGV);


__DATA__
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
