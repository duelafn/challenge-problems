#!/usr/bin/env perl6
use v6.c;

class Bot {
    has $.chip-limit = 2;
    has Int $.number;
    has $.chips = SetHash.new;
    has Bot $.low-target;
    has Bot $.high-target;

    method set-targets($!low-target, $!high-target) returns Bot { self }

    method give-chips() returns Bot {
        my ($a, $b);
        for $!chips.keys -> $chip {
            $a = $chip if !defined($a) or $chip < $a;
            $b = $chip if !defined($b) or $chip > $b;
        }
        put "$!number: $!chips => $!low-target.number() & $!high-target.number()";
        $!chips{$a, $b}:delete;
        $!low-target.take-chip($a);
        $!high-target.take-chip($b);
        return self;
    }

    method num-chips() returns Int { $!chips.elems }

    method take-chip(Int() $i) returns Bot {
        if $!chips.elems >= $!chip-limit {
            die "Bot $!number has too many chips @.chips + $i";
        }
        $!chips{$i} = 1;
        return self;
    }
}

class Output is Bot {
    has $.chip-limit = Inf;
}

class Factory {
    has %!bots;

    method add-rule($rule) {
        given $rule {
            when m:s/bot $<bot>=(\d+) gives low to $<low-type>=(bot|output) $<low>=(\d+) and high to $<high-type>=(bot|output) $<high>=(\d+)/ {
                return self.bot($<bot>, "bot").set-targets(self.bot($<low>, $<low-type>), self.bot($<high>, $<high-type>));
            }

            when m:s/value $<chip>=(\d+) goes to bot $<bot>=(\d+)/ {
                return self.bot($<bot>, "bot").take-chip($<chip>);
            }

            default {
                die "Unable to parse rule:\n$rule";
            }
        }
    }

    method bots()    { %!bots<bot>.values }
    method outputs() { %!bots<output>.values }

    method bot(Int() $num, $type = "unknown") {
        if %!bots{$type}{$num}:exists {
            return %!bots{$type}{$num};
        }
        given $type {
            when "bot"    { return %!bots{$type}{$num} = Bot.new(number => $num) }
            when "output" { return %!bots{$type}{$num} = Output.new(number => $num) }
            default       { die "Unknown bot type: $type" }
        }
    }
}

sub MAIN($file="10.in") {
    my $channel = Channel.new;
    my $factory = Factory.new;
    $factory.add-rule($_) for $file.IO.lines;
    for $factory.bots -> $bot {
        $channel.send($bot) if 2 == $bot.num-chips and $bot !~~ Output;
    }
    loop {
        my $bot = $channel.poll;
        last unless $bot;
        if 17 ∈ $bot.chips and 61 ∈ $bot.chips {
            put "Bot $bot.number() is processing chips 17 and 61";
        }
        $bot.give-chips;
        for $bot.low-target, $bot.high-target -> $target {
            $channel.send($target) if 2 == $target.num-chips and $target !~~ Output;
        }
    }
    say "{ .number }: { .chips }" for $factory.outputs.sort: { $^a.number <=> $^b.number };
    say [*] (^3).map: { $factory.bot(~$^i, "output").chips.pick };
}
