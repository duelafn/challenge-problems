#!/usr/bin/env perl6
use v6.c;

multi MAIN('rows', $file="03.in") {
    my $possible = 0;
    for $file.IO.lines -> $line {
        my @num = sort $line.comb(/\d+/)».Int;
        $possible++ if @num[0] + @num[1] > @num[2];
    }
    say $possible;
}

multi MAIN('cols', $file="03.in") {
    my $possible = 0;
    # $file.IO.lines           # read in file lines [ line1, line2, ... ]
    #    ».comb(/\d+/)         # for each line, produce a list from sequences of digits ( ("1", "2", "3"), ("4", "5", "6"), ... )
    #    ».Int                 # convert to integers so the sort works later ( (1, 2, 3), (4, 5, 6), ... )
    #                          # Note: ». works deeply so that we still get ints from our nested lists.
    #
    # ([Z] ...)                # @a Z @b produces @a[0] @b[0] @a[1] ...,
    #                          # In [Z], we end up with ( (1, 4, 7, ...), (2, 5, 8, ...), (3, 6, 9, ...))
    # flat([Z] ...)            # flatten so that our sub "-> $a, $b, $c { ... }" can pull off three at a time

    for flat([Z] $file.IO.lines».comb(/\d+/)».Int) -> $a, $b, $c {
        my @num = sort($a, $b, $c);
        $possible++ if @num[0] + @num[1] > @num[2];
    }
    say $possible;
}

multi MAIN('golf', $file="03.in") {
    say "rows: ", [+] $file.IO.lines.map:{my \n=.comb(/\d+/)».Int.sort;n[0]+n[1]>n[2]};

    # flat([Z] ...)            # as above, since flattened we have ( 1, 4, 7, ..., 2, 5, 8, ..., 3, 6, 9, ... )
    #    .rotor(3)             # group three-at-a-time ( (1, 4, 7), ... )
    #    ».sort                # sort each group as integers
    #    .map:{.[0]+.[1]>.[2]} # return True or False for each. Note: postcircumfix:<[ ]> is just a method so
    #                          #    we save chars by changing $_[0] to .[0] (which is just $_.[0])
    # [+] ...                  # just sum up the list, True|False get converted to 1|0
    say "cols: ", [+] flat([Z] $file.IO.lines».comb(/\d+/)».Int).rotor(3)».sort.map:{.[0]+.[1]>.[2]};
}
