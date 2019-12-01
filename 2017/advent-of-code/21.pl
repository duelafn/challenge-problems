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


package Art {
    use Moose;
    use MooseX::StrictConstructor;
    use Method::Signatures::Simple;
    use List::Util qw/ sum /;

    has grid => (is  => "ro", isa => "ArrayRef", default => sub { [
        '.#.',
        '..#',
        '###',
    ] });

    has rule_ref => (is  => "ro", isa => "HashRef", default => sub { {} });

    method show() { say for @{$self->grid}; }
    method size() { 0+@{$self->grid}; }
    method number_on() { sum( map 0+s/#/#/g, @{$self->grid} ) }

    method add_rule($in, $out) {
        $out = join "", grep /[.#]/, split //, $out;
        my @pat = grep /[.#]/, split //, $in;
        my $rules = $self->rule_ref;
        if (4 == @pat) { # D₄ on a 2×2
            my @img = map [split //], qw/ 0123 1302 3210 2031 2301 1032 0213 3120 /;
            $$rules{"@pat[@$_]"} = $out for @img;
        } else { # D₄ on a 3×3
            my @img = map [split //], qw/ 012345678 258147036 876543210 630741852 210543876 678345012 036147258 852741630 /;
            $$rules{"@pat[@$_]"} = $out for @img;
        }
    }

    method lookup($in) {
        return ($self->rule_ref->{$in} or die "Can't find pattern $in");
    }

    method key_at($x, $y, $size) {
        my @key;
        for my $o (0..($size-1)) {
            push @key, split //, substr($self->grid->[$y+$o], $x, $size);
        }
        return "@key";
    }

    method operate() {
        my @new;
        my $size = $self->size;
        my $step = 2 + ($size % 2);

        for my $x (0..($size/$step - 1)) {
            for my $y (0..($size/$step - 1)) {
                my @block = split //, $self->lookup($self->key_at($step*$x, $step*$y, $step));
                for my $i (0..$step) {
                    $new[(1+$step) * $y + $i] .= join "", map $block[(1+$step)*$i + $_], 0..$step;
                }
            }
        }
        @{$self->grid} = @new;
    }
}




func MAIN() {
    my $art;

    $art = Art->new;
    $art->add_rule(split / => /) for <DATA>;
    # $art->show; say "";
    $art->operate; # $art->show; say "";
    $art->operate; # $art->show; say "";
    is $art->number_on, 12, "test";

    $art = Art->new;
    $art->add_rule(split / => /) for lines("21.in");
    $art->operate for 1..5;
    is $art->number_on, 136, "part 1";

    $art = Art->new;
    $art->add_rule(split / => /) for lines("21.in");
    $art->operate for 1..18;
    is $art->number_on, 1911767, "part 2";
}



MAIN(@ARGV);
done_testing;

__DATA__
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
