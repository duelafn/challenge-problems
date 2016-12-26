use v6.c;

class Assembunny {
    has $.line is rw;
    has %.reg;
    has @.code;

    my token register { <[a..z]> }
    my token integer  { "-"? <digit>+ }

    method compile(@lines) {
        @!code.append: gather for @lines {
            when m:s/ cpy <value=integer> <to=register> /   { take ("cpy", +$<value>, ~$<to>); }
            when m:s/ cpy <from=register> <to=register> /   { take ("cpy", ~$<from>,  ~$<to>); }

            when m:s/ inc <register> /                      { take ("inc", ~$<register>); }
            when m:s/ dec <register> /                      { take ("dec", ~$<register>); }

            when m:s/ jnz <test=integer>  <goto=integer> /  { take ("jnz", +$<test>, +$<goto>); }
            when m:s/ jnz <test=register> <goto=integer> /  { take ("jnz", ~$<test>, +$<goto>); }
            when m:s/ jnz <test=integer>  <goto=register> / { take ("jnz", +$<test>, ~$<goto>); }
            when m:s/ jnz <test=register> <goto=register> / { take ("jnz", ~$<test>, ~$<goto>); }

            when m:s/ out <value=integer> /                 { take ("out", +$<value>); }
            when m:s/ out <value=register> /                { take ("out", ~$<value>); }

            when m:s/ tgl <value=integer> /                 { take ("out", +$<value>); }
            when m:s/ tgl <value=register> /                { take ("out", ~$<value>); }

            default { die "Not implemented" }
        }
        return self;
    }

    method init(:$line = 0, *%reg) {
        $!line = $line;
        %!reg{$_} = %reg{$_} // 0 for %!reg.keys;
    }

    method run()  { self.execute(|@!code[$!line]) while 0 <= $!line < @!code.elems }
    method step() { self.execute(|@!code[$!line]) }

    method registers(*@list) {
        %!reg{$_} = 0 for @list;
        return self;
    }

    method execute(Str $cmd, *@args) { self."$cmd"(|@args) }

    multi method cpy(Int $value, Str $reg) { %!reg{$reg} = $value; $!line++ }
    multi method cpy(Str $value, Str $reg) { self.cpy(%!reg{$value}, $reg) }

    multi method inc(Str $reg) { %!reg{$reg}++; $!line++ }
    multi method dec(Str $reg) { %!reg{$reg}--; $!line++ }

    multi method jnz(Int $test, Int $goto) { $!line += (($test and 0 <= $!line + $goto < @!code.elems) ?? $goto !! 1) }
    multi method jnz(Str $test, Int $goto) { self.jnz(%!reg{$test}, $goto) }
    multi method jnz(Int $test, Str $goto) { self.jnz($test, %!reg{$goto}) }
    multi method jnz(Str $test, Str $goto) { self.jnz(%!reg{$test}, %!reg{$goto}) }

    multi method out(Int $value) { print $value; $!line++ }
    multi method out(Str $value) { self.out(%!reg{$value}) }

    multi method tgl(Int $value) {
        my $cmd;
        given @!code[$!line + $value] {
            when "inc"       { $cmd = "dec" }
            when "dec"|"tgl" { $cmd = "inc" }
            when "jnz"       { $cmd = "cpy" }
            when "cpy"       { $cmd = "jnz" }
            default { die "Can't toggle command '$_'" }
        }
        @!code[$!line + $value] = ($cmd, |@!code[$!line + $value][1..*]);
        $!line++;
    }
    multi method tgl(Str $value) { self.tgl(%!reg{$value}) }
}
