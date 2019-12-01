#!/usr/bin/env perl6
use v6.c;

sub build_next($row) {
    state %trap = '^^.' => "^", '.^^' => "^", '^..' => "^", '..^' => "^";
    my $next = '.';
    $next ~= %trap{$row.substr($_-1, 3)} // "." for 1..($row.chars()-2);
    return "$next.";
}

multi MAIN(IO() $file="18.in", Int() :$n) { MAIN($file.lines[0], :$n) }

multi MAIN(Str $first where { not .IO ~~ :e }, Int() :$n) {
    my @floor = ".$first.";
    @floor.append: build_next(@floor[*-1]) while @floor.elems < $n;
    my $num_safe = -2 * @floor.elems;
    $num_safe += .comb('.').elems for @floor;
#     .say for @floor;
    say $num_safe;
}
