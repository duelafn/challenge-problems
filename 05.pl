#!/usr/bin/env perl6
use v6.c;
use experimental :pack;
use NativeCall;
sub _md5(Blob $data, uint64 $length, Blob $hash is rw) is native('ssl') is symbol('MD5') { * }

multi md5(Str $msg) returns Blob { md5($msg.encode("UTF-8")) }
multi md5(Blob $msg) returns Blob {
     my $digest = buf8.new(0 xx 16);
     _md5($msg, $msg.bytes, $digest);
     $digest;
}

sub md5_hex($msg) returns Str { md5($msg).unpack("H*") }

sub wanted(Blob $md5)  is pure { return 0 == $md5[0] && 0 == $md5[1] && $md5[2] < 16 }# 1 => 1a3099aa
sub wanted2(Blob $md5) is pure { return 0 == $md5[0] && 0 == $md5[1] && $md5[2] < 8  }# 2 => 694190cd

# sub wanted(Blob $md5)  is pure { return 0 == $md5[0] && $md5[1] < 4 }# 1 => ea87a6a1
# sub wanted2(Blob $md5) is pure { return 0 == $md5[0] && $md5[2] < 8 }# 2 => 5f55edca

multi MAIN('1', $code="uqwqemis") {
    my $md5 = buf8.new(0 xx 16);
    # Slightly faster to encode then concatenate than concatenate then encode:
    my Blob $codebuf = $code.encode("UTF-8");

    my $passwd;
    for 0..* -> $i {
        my Blob $msg = $codebuf ~ $i.Str.encode("UTF-8");
        _md5($msg, $msg.bytes, $md5);
        next unless wanted($md5);
        my $hash = $md5.unpack("H*");
        $passwd ~= $hash.substr(5, 1);
        last if 8 == $passwd.chars;
        LAST { put $passwd }
    }
}

multi MAIN('2', $code="uqwqemis") {
    my $md5 = buf8.new(0 xx 16);
    my Blob $codebuf = $code.encode("UTF-8");

    my @passwd = Nil xx 8;
    for 0..* -> $i {
        my Blob $msg = $codebuf ~ $i.Str.encode("UTF-8");
        _md5($msg, $msg.bytes, $md5);
        next unless wanted2($md5);
        my $hash = $md5.unpack("H*");
        @passwd[+$hash.substr(5, 1)] //= $hash.substr(6, 1);
        last if Nil ~~ none(@passwd);
        LAST { put @passwd.join("") }
    }
}
