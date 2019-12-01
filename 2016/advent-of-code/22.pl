#!/usr/bin/env perl6
use v6.c;

class Node {
    has Str $.id;
    has Int $.x;
    has Int $.y;
    has Int $.size;
    has Int $.used is rw;
    has Int $.avail is rw;

    method new(Int() :$x, Int() :$y, Int() :$size, Int() :$used, Int() :$avail) {
        return self.bless(:$x, :$y, :$size, :$used, :$avail, :id("($x,$y)"));
    }

    method clear_ids() { $!id = "" }

    method gist() { "\{$!x,$!y\} $!id" }

    method changed() { $!id ne "($!x,$!y)" }

    method add_ids($ids) {
        my @ids = $!id.comb(/\S+/);
        @ids.append: $ids.comb(/\S+/);
        $!id = join " ", @ids.sort.grep: { /\S/ }
    }
}

class Cluster {
    has $.rows = 0;
    has $.cols = 0;
    has @.node-array;
    has @.moves;

    method clone() {
        my $clone = self.new(:rows($!rows), :cols($!cols), :moves(@!moves));
        for ^$!rows X, ^$!cols -> [$x, $y] {
            $clone.node-array[$y;$x] = @!node-array[$y;$x].clone;
        }
        return $clone;
    }

    method id() { self.nodes.grep({ .changed })».gist.join(";") }
    multi method at(Node $node --> Node) is pure     { @!node-array[$node.y;$node.x] }
    multi method at(Int $x, Int $y --> Node) is pure { @!node-array[$y;$x] }

    method add(Node $node) {
        $!rows = $node.y + 1 if $node.y >= $!rows;
        $!cols = $node.x + 1 if $node.x >= $!cols;
        @!node-array[$node.y;$node.x] = $node;
        return self;
    }

    method clone-move(Node $a, Node $b) {
        my $clone = self.clone;
        $clone.move($clone.at($a), $clone.at($b));
        return $clone;
    }

    method move(Node $a, Node $b) {
        @!moves.append: "($a.x(),$a.y()) -> ($b.x(),$b.y())";
        $b.used += $a.used;
        $b.avail = $b.size - $b.used;
        $b.add_ids($a.id);
        $a.used = 0;
        $a.avail = $a.size;
        $a.clear_ids();
        return self;
    }

    method nodes() {
        gather for @!node-array -> @row {
            for @row { take $_ if .defined }
        }
    }

    method viable-pairs() {
        gather for self.nodes.combinations(2) -> [ $a, $b ] {
            take ($a, $b) if $a.used > 0 and $a.used <= $b.avail;
            take ($b, $a) if $b.used > 0 and $b.used <= $a.avail;
        }
    }

    multi method viable-moves(Node $node) {
        self.viable-moves($node.x, $node.y)
    }

    multi method viable-moves($x, $y) {
        my $node = self.at($x, $y);
        gather for self.neighbors($x, $y) -> $b {
            take ($node, $b) if $node.used > 0 and $node.used <= $b.avail;
        }
    }

    multi method neighbors(Node $node) is pure {
        self.neighbors($node.x, $node.y)
    }

    multi method neighbors(Int $x, Int $y) is pure {
        gather {
            take @!node-array[$y-1;$x] if $y > 0 and @!node-array[$y-1;$x];
            take @!node-array[$y+1;$x] if @!node-array[$y+1;$x];
            take @!node-array[$y;$x-1] if $x > 0 and @!node-array[$y;$x-1];
            take @!node-array[$y;$x+1] if @!node-array[$y;$x+1];
        }
    }
}

sub load($file) {
    my $cluster = Cluster.new;
    for $file.IO.lines {
        when / ^ "#" / { 1; }
        when / "node-x" $<x>=(\d+) "-y" $<y>=(\d+) <ws> $<size>=(\d+)T <ws> $<used>=(\d+)T <ws> $<avail>=(\d+)T / {
            $cluster.add( Node.new(|$/.hash) );
        }
        default { die "Can't parse $_" }
    }
    return $cluster;
}

sub MAIN(IO() $file="22.in", :$wanted = "(32,0)") {
    my @todo = load($file);
    my %seen;
    while @todo {
        my $cluster = @todo.shift;
        next if %seen{$cluster.id}++;
        if $cluster.at(0,0).id.contains($wanted) {
            .say for $cluster.moves;
            say "Finish in { $cluster.moves.elems } moves";
            exit;
        }

        for $cluster.nodes -> $node {
            for $cluster.viable-moves($node) -> [ $a, $b ] {
                @todo.append: $cluster.clone-move($a, $b);
            }
        }
        say "TODO: ", @todo.elems;
    }
}
