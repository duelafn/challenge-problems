#!/usr/bin/env perl6
use v6.c;

my regex ABBA {
    (\w) (\w) <?{ $0 ne $1 }> $1 $0
}

my regex ABA {
    (\w) (\w) <?{ $0 ne $1 }> $0
}

sub parse-ip($ip) {
    $ip.comb(/ "[" <alpha>+ "]" | <alpha>+ /)\
    .classify: {
        .starts-with("[") ?? "hypernet" !! "supernet"
    }
}

multi sub MAIN('ABBA', $file="07.in", Bool :$verbose) {
    my $count = 0;
    LEAVE { put $count }
    for $file.IO.lines -> $line {
        my %piece = parse-ip($line);
        next if     any(%piece<hypernet>) ~~ /<ABBA>/;
        next unless any(%piece<supernet>) ~~ /<ABBA>/;
        put $line if $verbose;
        $count++;
    }
}

multi sub MAIN('SSL', $file="07.in", Bool :$verbose) {
    my $count = 0;
    LEAVE { put $count }
    for $file.IO.lines -> $line {
        my %piece = parse-ip($line);

        for %piece<supernet>.map({ m:g:ex/(<ABA>)/.Slip if $_ }) -> $match {
            next unless $match;
            my ($a, $b) = $match.comb;
            if any(%piece<hypernet>) ~~ / $b $a $b / {
                put $line if $verbose;
                $count++;
                last;
            }
        }
    }
}
