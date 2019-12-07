#!/usr/bin/perl -w
use strict; use warnings; use 5.020;
no warnings "experimental::signatures";
use feature 'signatures';

package Node {
    sub new($class) {
        bless { parent => undef, children => [] }, $class;
    }
}

package KeyTree {
    sub new($class) {
        bless { nodes => {}, data => {} }, $class;
    }

    sub contains_node($self, $key) { exists($$self{nodes}{$key}); }
    sub set_node($self, $key, $data) {
        $$self{data}{$key} = $data;
        $$self{nodes}{$key} //= Node->new;
    }

    sub add_edge($self, $a, $b) {
        die "MissingSource"      unless exists($$self{nodes}{$a});
        die "MissingDestination" unless exists($$self{nodes}{$b});
        die "DuplicateParent"    if $$self{nodes}{$b}{parent};
        $$self{nodes}{$b}{parent} = $a;
        push @{$$self{nodes}{$a}{children}}, $b;
    }

    sub get($self, $key)      { $$self{data}{$key} }
    sub parent($self, $key)   { return unless exists($$self{nodes}{$key}); $$self{nodes}{$key}{parent} }
    sub children($self, $key) { return unless exists($$self{nodes}{$key}); @{$$self{nodes}{$key}{children}} }

    sub roots($self)  { grep { !$$self{nodes}{$_}{parent} } keys %{$$self{nodes}} }
    sub leaves($self) { grep { !@{$$self{nodes}{$_}{children}} } keys %{$$self{nodes}} }

    sub find_path($self, $a, $b) {
        die "No such node" unless exists($$self{nodes}{$a}) and exists($$self{nodes}{$b});
        my $best = 0+keys(%{$$self{nodes}});
        my @best_path;
        my @todo = ([[$a], $$self{nodes}{$a}, {$a=>1}]);
        while (@todo) {
            my ($path, $A, $seen) = @{pop @todo};
            for my $nxt ($$A{parent}, @{$$A{children}}) {
                next unless defined($nxt);
                if ($nxt eq $b) {
                    if (!@best_path or $best > 1+@$path) {
                        @best_path = (@$path, $b);
                        $best = @best_path;
                    }
                    next;
                }
                next if $$seen{$nxt};
                next if 1+@$path > $best;
                push @todo, [ [@$path, $nxt], $$self{nodes}{$nxt}, { %$seen, $nxt => 1 } ];
            }

        }
        return @best_path;
    }

    sub climb($self) {
        my @todo = $self->roots;
        my %seen;
        my @nodes;
        while (@todo) {
            my $base = shift @todo;
            next if $seen{$base}++;
            push @nodes, $base;
            push @todo, @{$$self{nodes}{$base}{children}};
        }
        return reverse(@nodes);
    }
}


my $tree = KeyTree->new;
for (<<>>) {
    die "Parse error in '$_'" unless /^(\w+)\)(\w+)$/;
    my ($a, $b) = ($1, $2);
    $tree->set_node($a, 0) unless $tree->contains_node($a);
    $tree->set_node($b, 0) unless $tree->contains_node($b);
    $tree->add_edge($a, $b);
}

my $total_orbits = 0;
for my $node ($tree->climb) {
    my $count = 0;
    for my $chld ($tree->children($node)) {
        $count += 1 + ($tree->get($chld) // 0);
    }

    $tree->set_node($node, $count);
    $total_orbits += $count;
}

say "Total orbits: $total_orbits";

my @path = $tree->find_path("YOU", "SAN");
say "Path to SAN: " . (@path - 3);
