#!/usr/bin/env perl6
use v6.c;

# TODO: Ideally, we'd build QAST from a grammar, but with no concrete API
#       don't really want to do that.
class VM {
    has Int %.reg;
    has @.code;
    has $.line = 0;

    my token register { <[abcd]> }
    my token integer  { "-"? <digit>+ }

    method step() {
        given @!code[$!line] {
            when m:s/ cpy <value=integer> <to=register> /  { %!reg{~$<to>} = +$<value>; $!line++ }
            when m:s/ jnz <test=register> <goto=integer> / { $!line += (0 == (%!reg{~$<test>} //= 0) ?? 1 !! $<goto>) }
            when m:s/ dec <register> /                     { %!reg{~$<register>}--; $!line++ }
            when m:s/ inc <register> /                     { %!reg{~$<register>}++; $!line++ }
            when m:s/ cpy <from=register> <to=register> /  { %!reg{~$<to>} = (%!reg{~$<from>} //= 0); $!line++ }
            when m:s/ jnz <test=integer> <goto=integer> /  { $!line += (0 == $<test> ?? 1 !! $<goto>) }
            default { die "Not implemented" }
        }
    }

    method compile(@registers, @lines) {
        %!reg{~$_} = 0 for @registers;
        for @lines {
            when m:s/ cpy <value=integer> <to=register> / {
                my $value = +$<value>;
                my $reg := %!reg{~$<to>};
                @!code.push: { $reg = $value; $!line++ }
            }
            when m:s/ cpy <from=register> <to=register> / {
                my $src := %!reg{~$<from>};
                my $tgt := %!reg{~$<to>};
                @!code.push: { $tgt = $src; $!line++ }
            }
            when m:s/ inc <register> / {
                my $reg := %!reg{~$<register>};
                @!code.push: { $reg++; $!line++ }
            }
            when m:s/ dec <register> / {
                my $reg := %!reg{~$<register>};
                @!code.push: { $reg--; $!line++ }
            }
            when m:s/ jnz <test=integer> <goto=integer> / {
                my $step = $<goto>;
                my $test = +$<test>;
                @!code.push: { $!line += (0 == $test ?? 1 !! $step) };# NOTE: Don't optimize this away, else breaks jumps across it
            }
            when m:s/ jnz <test=register> <goto=integer> / {
                my $step = $<goto>;
                my $reg := %!reg{~$<test>};
                @!code.push: { $!line += (0 == $reg ?? 1 !! $step) }
            }
            default { die "Not implemented" }
        }
        return self;
    }

    method run($!line = 0) {
        my $start = BEGIN now;
        my $steps = 0;
        LEAVE { my $t = now; say "$steps steps in { ($t - $start).Int } seconds ({ ($steps / ($t - $start)).Int } ips)" }
        while $!line < @!code.elems {
            @!code[$!line]();
            $steps++;
        }
    };

    method halted() { $!line > @.code.end }
    method dump()   { say %!reg }
}

#   954509 steps in   7 seconds (120730 ips)
# 27683521 steps in 227 seconds (121619 ips)
multi MAIN('bin', $file="12.in", Bool :$verbose, :$prepend) {
    my @lines = $file.IO.lines;
    @lines.prepend: $prepend if $prepend;
    my $vm = VM.new.compile(['a'..'d'], @lines);
    $vm.run;
    $vm.dump;
}

#   954509 steps in 151 seconds (6307 ips)
multi MAIN('interp', $file="12.in", Bool :$verbose, :$prepend) {
    my $start = BEGIN now;
    my $steps = 0;
    LEAVE { my $t = now; say "$steps steps in { ($t - $start).Int } seconds ({ ($steps / ($t - $start)).Int } ips)" }
    my @lines = $file.IO.lines;
    @lines.prepend: $prepend if $prepend;
    my $vm = VM.new(code => @lines);
    while not $vm.halted {
        $vm.step;
        $steps++;
    }
    $vm.dump unless $verbose;
}
