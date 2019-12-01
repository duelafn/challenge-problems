#!/usr/bin/env perl6
use v6.c;

use lib '.';
use Assembunny;

class Assembunny25 is Assembunny {
    has $.emmissions = 0;

    method init(|c) {
        $!emmissions = 0;
        nextsame;
    }

    multi method out(Int $value) {
        die "Wrong sequence" if $value != $!emmissions++ % 2;
        say "Emitted $!emmissions" if $!emmissions %% 100;
        $.line++;
    }
}

sub MAIN(IO() $file="25.in") {
    my $interp = Assembunny25.new.registers('a'..'d').compile($file.lines);

    for 150..* -> $a {
        my $local = $interp.clone;
        put "Test \$a == $a";
        $local.init(:$a);
        try {
            CATCH { default { "ok" } }
            $local.run
        };
    }
}
