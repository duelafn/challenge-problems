#!/usr/bin/env perl6
use v6.c;

# perl -E 'chomp($_ = <>); s/\((\d+)x(\d+)\)(.{\1})/$3 x $2/e; say length'

sub decompress($content is copy, :$version=1) {
    loop {
        # Perl6 bug prevents simple capture of . ** {$0}
        last unless $content ~~ s:g/ "(" (\d+) x (\d+) ")" $<rv>=[ . ** {$0} ] /{ $<rv> x $1 }/;
        last if 1 == $version;
    }
    return $content;
}

sub decompressed-length($content, :$version=1) {
    my $length = 0;
    my $pos = 0;
    while $pos < $content.chars {
        if $content ~~ m:p($pos)/ "(" (\d+) x (\d+) ")" $<rv>=[ . ** {$0} ] / {
            $pos = $/.to;
            $length += $1 * (1 == $version ?? $<rv>.chars !! decompressed-length($<rv>, :$version));
        }

        elsif $content ~~ m:p($pos)/ .*? [ <before "(" \d+ x \d+ ")" > | $ ] / {
            $pos = $/.to;
            $length += $/.chars;
        }

        else {
            die "Parse error at column $pos:\n$content\n" ~ " " x $pos ~ "^-- HERE";
        }
    }
    return $length;
}


sub MAIN($file="09.in") {
#     put decompress("ADVENT");
#     put decompress("A(1x5)BC");
#     put decompress("(3x3)XYZ");
#     put decompress("(6x1)(1x3)A");
#     put decompress("X(8x2)(3x3)ABCY");
#     put decompress("(27x12)(20x12)(13x14)(7x10)(1x12)A", :version(2)).chars;
#     put decompress("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", :version(2)).chars;
#     put decompress($_, :version(1)).chars for $file.IO.lines;
#     put decompress($_, :version(2)).chars for $file.IO.lines;
#     put decompressed-length("(27x12)(20x12)(13x14)(7x10)(1x12)A", :version(2));
#     put decompressed-length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", :version(2));
    put decompressed-length($_, :version(1)) for $file.IO.lines;
    put decompressed-length($_, :version(2)) for $file.IO.lines;
}
