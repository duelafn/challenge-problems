#!/usr/bin/env perl6
use v6.c;

# TODO: try compiling to anon subs in @.code (perhaps faster since no parsing)
#
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
            when m:s/ jnz <test=register> <goto=integer> / { $!line += $<goto> unless 0 == (%!reg{~$<test>} //= 0) }
            when m:s/ dec <register> /                     { %!reg{~$<register>}--; $!line++ }
            when m:s/ inc <register> /                     { %!reg{~$<register>}++; $!line++ }
            when m:s/ cpy <from=register> <to=register> /  { %!reg{~$<to>} = (%!reg{~$<from>} //= 0); $!line++ }
            when m:s/ jnz <test=integer> <goto=integer> /  { $!line += $<goto> unless 0 == $<test> }
            default { die "Not implemented" }
        }
    }

    method halted() { $!line > @.code.end }
    method dump()   { say %!reg }
}


sub MAIN($file="12.in", Bool :$verbose) {
    my $vm = VM.new(code => [$file.IO.lines]);
    while not $vm.halted {
        $vm.step;
        $vm.dump if $verbose;
    }
    $vm.dump unless $verbose;
}
