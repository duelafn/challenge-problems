#!/usr/bin/perl

use 5.020;
use Digest::MD5 qw/ md5_hex /;

say md5_hex("abcdef609043");

my $i = 0;
my $pre = "bgvyzdsv";
my $hash = '';

until (substr($hash, 0, 5) eq '00000') {
    $i++;
    $hash = md5_hex("$pre$i");
}
say "$i: $hash";

until (substr($hash, 0, 6) eq '000000') {
    $i++;
    $hash = md5_hex("$pre$i");
}
say "$i: $hash";
