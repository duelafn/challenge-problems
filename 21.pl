#!/usr/bin/env perl6
use v6.c;

sub apply($cmd, @str) {
    given $cmd {
        when m:s/swap position (\d+) with position (\d+)/ {
            @str[$0, $1] = @str[$1, $0]
        }

        when m:s/swap letter (\w) with letter (\w)/ {
            my ($a, $b) = ~$0, ~$1;
            @str = gather for @str {
                when $a { take $b }
                when $b { take $a }
                default { take $_ }
            }
        }

        when m:s/rotate left (\d+) step/ {
            @str.=rotate(+$0);
        }

        when m:s/rotate right (\d+) step/ {
            @str.=rotate(-$0);
        }

        when m:s/rotate based on position of letter (\w)/ {
            my $idx = @str.first(~$0, :k);
            @str.=rotate( -(1 + $idx + ($idx >= 4)) );
        }

        when m:s/reverse positions (\d+) through (\d+)/ {
            @str[$0..$1] = @str[($0..$1).reverse]
        }

        when m:s/move position (\d+) to position (\d+)/ {
            @str.splice(+$1, 0, @str.splice(+$0, 1))
        }

        default {
            die "Unknown command: '$cmd'";
        }
    }
}

sub unapply($cmd, @str) {
    # Most are their own inverses
    given $cmd {
        when m:s/swap position (\d+) with position (\d+)/ {
            @str[$0, $1] = @str[$1, $0]
        }

        when m:s/swap letter (\w) with letter (\w)/ {
            my ($a, $b) = ~$0, ~$1;
            @str = gather for @str {
                when $a { take $b }
                when $b { take $a }
                default { take $_ }
            }
        }

        when m:s/rotate left (\d+) step/ {
            @str.=rotate(-$0);
        }

        when m:s/rotate right (\d+) step/ {
            @str.=rotate(+$0);
        }

        # Hm, ASSUME only single instance of such a letter
        #
        # BOO, not always unique!
        #    $ ./21.pl eval 'rotate based on position of letter d' abdec
        #    Rotate right: 3
        #    decab
        #    $ ./21.pl eval 'rotate based on position of letter d' ecabd
        #    Rotate right: 6
        #    decab
        #
        # Running on the input I have works out though.
        when m:s/rotate based on position of letter (\w)/ {
            my $idx = @str.first(~$0, :k);
            # Probably could be more clever than this, but don't have time:
            my $old = (0..@str.end).first: {
                $idx == (2 * $_ + 1 + ($_ >= 4)) % @str.elems
            }
            @str.=rotate($idx-$old);
        }

        when m:s/reverse positions (\d+) through (\d+)/ {
            @str[$0..$1] = @str[($0..$1).reverse]
        }

        when m:s/move position (\d+) to position (\d+)/ {
            @str.splice(+$0, 0, @str.splice(+$1, 1))
        }

        default {
            die "Unknown command: '$cmd'";
        }
    }
}

multi MAIN(Str $passwd="abcdefgh", IO() $file="21.in", Bool :$reverse, Bool :$verbose) {
    my @passwd = $passwd.comb;
    my @lines = $file.lines;
    @lines = @lines.reverse if $reverse;

    for @lines -> $cmd {
        if $reverse { unapply($cmd, @passwd) }
        else        { apply($cmd, @passwd) }
        say "$cmd -> @passwd[]" if $verbose;
    }
    put join "", @passwd;
}

multi MAIN('eval', Str() $cmd, Str $passwd="abcdefgh", Bool :$reverse, Bool :$verbose) {
    my @passwd = $passwd.comb;
    if $reverse { unapply($cmd, @passwd) }
    else        { apply($cmd, @passwd) }
    put join "", @passwd;
}
