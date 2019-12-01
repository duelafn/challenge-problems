#!/usr/bin/env perl6
use v6.c;

sub checksum($name) {
    Bag.new($name.comb(/<[a..z]>/)).pairs.sort({ $^b.value <=> $^a.value || $^a.key cmp $^b.key })[^5].map(*.key).join
}

sub caesar($text, $rot is copy) {
    $rot %= 26;
    $text.ords.map({ if $_ == 45 { 32 } else { my $v = $_ + $rot; $v -= 26 if $v > 122; $v } })».chr.join
}

sub MAIN($file = '04.in') {
    my $sum = 0;
    for $file.IO.lines {
        die "Unexpected input $_" unless / $<name>=(<[\w-]>+) "-" $<id>=(\d+) "[" $<check>=(\w+) "]" /;
        my ($name, $id, $check) = $/<name id check>;
        next unless checksum($name) eq $check;
        $sum += $id;
        my $caesar = caesar($name, $id);
        say "$caesar: $id" if $caesar ~~ /north/;
    }
    say "Sum: $sum";
}
