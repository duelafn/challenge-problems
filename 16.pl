#!/usr/bin/env perl6
use v6.c;

# Native Arrays
#--------------
# Fastest and best memory use (but still too much)
multi dragon(int @a) {
    my int @d = @a;
    @d.append: 0, @a.reverse.map: { $_ ?? 0 !! 1 };
    return @d;
}

multi dragon-stretch(int @a, $length) {
    return @a if @a.elems >= $length;
    my int @b := dragon(@a);
    while @b.elems <= $length {
        @b := dragon(@b);
    }
    return @b[^$length];
}

multi checksum(@a) {
    return @a if @a.elems % 2;
    my int @c[@a.elems/2];
    for ^@c.elems -> $i {
        @c[$i] = (@a[2*$i] == @a[2*$i+1] ?? 1 !! 0);
    }
    return checksum(@c);
}

# Strings
#--------
# Slow and consume all my memory (on second part)
multi dragon(Str $a) {
    $a ~ "0" ~ $a.flip.trans("01" => "10")
}

multi dragon-stretch(Str $a is copy, $length) {
    $a = dragon($a) while $a.chars < $length;
    return $a.substr(0, $length);
}

# This should be fastest, but Str.trans is rather complex and builds big
# lists, so ends up unusable.
multi checksum(Str $str is copy) {
    $str = $str.trans(["00", "11", "10", "01"] => ["1", "1", "0", "0"]) while $str.chars %% 2;
    return $str;
}

# This turns out to be best of the ones here, but still unusable.
multi checksum2(Str $str is copy) {
    state %repl = ( "00" => "1", "11" => "1", "01" => "0", "10" => "0" );
    while $str.chars %% 2 {
        $str ~~ s:g/(<[01]><[01]>)/{ %repl{~$0} }/;
    }
    return $str;
}

multi checksum3(Str $str is copy) {
    while $str.chars %% 2 {
        $str ~~ s:g/(.)(.)/{ $0 eq $1 ?? "1" !! "0" }/;
    }
    return $str;
}

multi checksum4(Str $str) {
    if $str.chars %% 2 {
        my $rv = "0" x ($str.chars / 2);
        for ^$rv.chars -> $i {
            $rv.substr-rw($i, 1) = "1" if $str.substr(2*$i, 1) eq $str.substr(2*$i+1, 1);
        }
        return checksum4($rv);
    }
    return $str;
}


sub MAIN(Str() $initial, Int() :$length=0) {
    my int @a = $initial.comb.map: *.Int;
    @a = dragon-stretch(@a, $length);
    say join("", checksum(@a));
}
