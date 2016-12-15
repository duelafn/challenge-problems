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

sub stretch(Str $msg) returns Str {
    my $a = buf8.new(0 xx 16);
    _md5($msg.encode("ISO-8859-1"), $msg.chars, $a);
    _md5($a.unpack("H*").encode("ISO-8859-1"), 32, $a) for ^2016;
    return $a.unpack("H*");
}

my %TYPE = ( :plain(&md5_hex), :stretch(&stretch) );
sub MAIN(Str $type where { $_ ~~ 'plain'|'stretch' }, $salt="zpqevtbw") {
    my &hash = %TYPE{$type};
    my @keep;

    ## Alas, with current perl6 the following is actually a small bit
    ## slower than the non-hyper version when running 'plain'. If the bug
    ## in hyper didn't cause failures, we'd probably see a win in the
    ## 'stretch' case.
    ##
    ## Bugs in hyper make this sometimes crash (https://rt.perl.org/Public/Bug/Display.html?id=127452)
    #
    # Compute first thousand in parallel:
#     my @q = [0..^1000].hyper(:batch(125), :degree(8)).map({ hash($salt ~ $^idx) });
    my @q = (^1000).map({ hash($salt ~ $^idx) });
    my $idx = 1000;

    # The parallel version is again slower, not sure if this is something
    # that will get fixed later - it isn't as obvious a win as the .hyper
    # above.
    my $promise = start { hash($salt ~ $idx++) }
    while @keep.elems < 64 {
        my $hash = shift @q;
        @q.push: $promise.result;
        $promise = start { hash($salt ~ $idx++) }
        next unless $hash ~~ / (.) $0 $0 /;
        my $search = $0 x 5;
        ## Same speed as index(join(" ", @q), $0 x 5), which is surprising
        for @q {
            with .index($search) {
                push @keep, $hash;
                last;
            }
        }
    }
    await $promise;
    # -1002 since we compute an extra with the last parallel promise:
    put $idx - 1002;
}
