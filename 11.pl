#!/usr/bin/env perl6
use v6.c;

my %ORDINAL = first => 1, second => 2, third => 3, fourth => 4;

class Object {
    has $.type;
    method gist() { "$!type { self.WHO }" }
}
class Microchip is Object { }
class Generator is Object { }

class State {
    has $.step = 0;
    has SetHash @.floors;
    has $.elevator = 1;
    has $.min-floor = 1;
    has $.max-floor = 4;

    method place(Int $floor, Object $obj) {
        self.floor($floor){$obj} = 1;
    }

    method floor(Int $floor) { @!floors[$floor] //= SetHash.new }

    method is-safe() returns Bool {
        for $!min-floor..$!max-floor -> $floor {
            my %items = @!floors[$floor].keys.classify({~ .WHO});
            next unless %items<Generator> and %items<Microchip>;
            for |%items<Microchip> -> $chip {
                return False unless $chip.type eq any(|%items<Generator>».type);
            }
        }
        return True;
    }

    method uid() {
        my @rv;
        for $!min-floor..$!max-floor -> $floor {
            @rv.append("$floor { $!elevator == $floor ?? "*" !! ":" } { join ", ", self.floor($floor).keys».gist.sort }");
        }
        join "\n", @rv;
    }

    method gist() {
        "STEP $!step\n" ~ self.uid
    }

    method move(Int $delta where { $delta == +1|-1 }, *@stuff) {
        self.floor($!elevator){@stuff}:delete;
        $!elevator += $delta;
        self.floor($!elevator){$_} = 1 for @stuff;
        $!step++;
        return self;
    }

    method all-moves() {
        self.floor($!elevator).keys.combinations(1..2).map: {
            gather {
                take self.clone.move(+1, $_.Slip) if $!elevator < $!max-floor;
                take self.clone.move(-1, $_.Slip) if $!elevator > $!min-floor;
            }.Slip
        }
    }

    method safe-moves() {
        self.all-moves.grep(*.is-safe);
    }

    method clone() {
        self.bless(
            step => $!step,
            elevator => $!elevator,
            floors => @!floors.map({ SetHash.new($^f.keys) }),
            min-floor => $!min-floor,
            max-floor => $!max-floor,
        )
    }
}

sub load-state($file) {
    my $state = State.new(min-floor => 1, max-floor => 4);
    for $file.IO.lines {
        die unless m:s/The (\w+) floor/;
        my $floor = %ORDINAL{$0};
        for m:g/ <ws> an? <ws> (.*?) ("-compatible microchip"|" generator")/ -> $/ {
            my $obj = $1 eq ' generator'
                ?? Generator.new(type => ~$0)
                !! Microchip.new(type => ~$0);
            $state.place($floor, $obj);
        }
    }
    return $state;
}

sub wanted($state) {
    for $state.min-floor..^$state.max-floor -> $i {
        return False unless $state.floor($i).elems == 0;
    }
    return True
}

# TODO: How to do this in parallel?
sub MAIN($file="11.in", Bool :$verbose) {
    my $channel = Channel.new;
    my $state = load-state($file);
    my %seen = $state.uid => 1;
    $channel.send($state);
    loop {
        $state = $channel.poll;
        last unless $state;
        say $state if $verbose;
        if wanted($state) {
            say $state unless $verbose;
            exit;
        }
        $channel.send($_) for $state.safe-moves.grep({ !(%seen{.uid}++) });
    }
}
