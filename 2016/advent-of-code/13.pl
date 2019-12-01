#!/usr/bin/env perl6
use v6.c;

subset NonnegInt of Int where * >= 0;

# A path that only remembers its length, not where it has been.
class ForgetfulPath {
    has $.x = 1;
    has $.y = 1;
    has $.length = 0;

    method gist() {
        "($!x, $!y) after $!length steps";
    }

    method step(Int $dx, Int $dy) {
        $!x += $dx;
        $!y += $dy;
        $!length++;
        return self;
    }
}

class Maze {
    has $.formula is required;
    has @.map;

    method square-type(NonnegInt() $x, NonnegInt() $y) {
        @!map[$y][$x] //= self.compute-type($x, $y);
    }

    method compute-type(NonnegInt() $x, NonnegInt() $y) {
        $!formula($x, $y).base(2).comb("1").elems % 2 ?? "#" !! " ";
    }

    method is-wall(NonnegInt() $x, NonnegInt() $y) { self.square-type($x, $y) eq "#" }
    method is-open(NonnegInt() $x, NonnegInt() $y) { self.square-type($x, $y) eq " " }

    method seen(NonnegInt() $x, NonnegInt() $y) {
        defined(@!map[$y][$x])
    }

    method gist() {
        my @rv;
        my $cols = max(@!map».elems);
        for ^@!map.end -> $y {
            if @!map[$y] {
                push @rv, join "", (^$cols).map: -> $x { defined(@!map[$y][$x]) ?? @!map[$y][$x] !! "?" };
            } else {
                push @rv, "?"x$cols;
            }
        }
        join "\n", @rv;
    }
}

multi MAIN(Str $action where { $_ eq 'goto'|'reachable' }, Int() $input=1358, Int() :$after=50, Int() :$x=31, Int() :$y=39, Bool :$verbose) {
    my $maze = Maze.new( :formula(-> $x, $y { $x * $x + 3 * $x + 2 * $x * $y + $y + $y * $y + $input}) );
    my $channel = Channel.new;
    my $pos = ForgetfulPath.new;
    die "Teleported into a wall!" unless $maze.is-open($pos.x, $pos.y);
    $channel.send($pos);
    my $reachable = 0;
    loop {
        $pos = $channel.receive;
        say $pos if $verbose;

        if $action eq 'reachable' and $pos.length > $after {
            say $maze;
            say "$reachable locations are reachable within $after steps";
            exit;
        }
        $reachable++;

        if $action eq 'goto' and $x == $pos.x and $y == $pos.y {
            say $pos  unless $verbose;
            say $maze;
            exit;
        }

        $channel.send($_) for gather for (1,0), (0,1), (-1,0), (0,-1) -> [$i, $j] {
            my ($x, $y) = $pos.x+$i, $pos.y+$j;
            if ($x >= 0
                and $y >= 0
                and not $maze.seen($x, $y)
                and $maze.is-open($x, $y)
               ) {
                take $pos.clone.step($i, $j);
            }
        }
    }
}
