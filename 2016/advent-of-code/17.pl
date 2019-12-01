#!/usr/bin/env perl6
use v6.c;
use experimental :pack;

use NativeCall;
sub  _md5(Blob $data, uint64 $length, Blob $hash is rw) is native('ssl') is symbol('MD5') { * }
sub   md5_hex($msg)  returns Str  { md5($msg).unpack("H*") }
multi md5(Str $msg)  returns Blob { md5($msg.encode("ISO-8859-1")) }
multi md5(Blob $msg) returns Blob {
    my $digest = buf8.new(0 xx 16);
    _md5($msg, $msg.bytes, $digest);
    $digest;
}

class Walker {
    has $.rows = 4;
    has $.cols = 4;
    has $.salt;
    has $.path = '';
    has $.x = 1;
    has $.y = 1;

    method length() { $!path.chars }

    method valid() {
        my ($a, $b, $c, $d) = md5_hex($!salt ~ $!path).comb;
        return gather {
            take "U" if $!y > 1      and :16($a) > 10;
            take "D" if $!y < $!rows and :16($b) > 10;
            take "L" if $!x > 1      and :16($c) > 10;
            take "R" if $!x < $!cols and :16($d) > 10;
        }
    }

    method step($d) {
        $!path ~= $d;
        given $d {
            when 'U' { $!y-- }
            when 'D' { $!y++ }
            when 'L' { $!x-- }
            when 'R' { $!x++ }
        }
        return self;
    }
}

sub MAIN(Str $salt="bwnlcvfs") {
    my @todo = Walker.new(:$salt);
    my $path;
    while @todo {
        my $w = shift @todo;
        if $w.x == 4 and $w.y == 4 {
            put "First: $w.path()" unless $path;
            $path = $w;
        }
        else {
            @todo.append: $w.valid.map: { $w.clone.step($^d) }
        }
    }
    put $path.?length || "Bummer";
}
