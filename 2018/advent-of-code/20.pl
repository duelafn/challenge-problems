#!/usr/bin/env perl6
use v6.c;

grammar MapPath {
    token TOP      { ^ '^' <sequence>+ '$' \s* $ }
    token sequence { <chunk>+ }
    token chunk    { <literal> || <options> }
    token literal  { <[NSEW]> }
    token options  { '(' [ <sequence>* %% '|' ] ')' }
}

sub MAIN(IO() $file="20.in") {
    say MapPath.parse($file.slurp);
}
