#!/usr/bin/python3 -b
# -*- coding: utf-8 -*-
import sys

to_win  = dict(R="P", P="S", S="R")
to_lose = dict(R="S", P="R", S="P")
to_tie  = dict(R="R", P="P", S="S")

weights = dict(R=1, P=2, S=3)
def score(us, them):
    if us == them:          return weights[us] + 3
    if is_winner(us, them): return weights[us] + 6
    return weights[us]

def is_winner(us, them):
    return (us, them) in ( ("R", "S"), ("P", "R"), ("S", "P") );


def part_1(strategy):
    map = dict(A="R", B="P", C="S", X="R", Y="P", Z="S")
    total = 0
    for (them, us) in strategy:
        total += score(map[us], map[them])
    print(f"Part 1: {total}")


def part_2(strategy):
    map = dict(A="R", B="P", C="S")
    total = 0
    guide = dict(X=to_lose, Y=to_tie, Z=to_win)
    for (them, us) in strategy:
        total += score(guide[us][map[them]], map[them])
    print(f"Part 2: {total}")


def load(fname):
    rv = []
    with open(fname) as fh:
        for line in fh:
            rv.append(line.split())
    return rv


strategy = load(sys.argv[1] if 2 == len(sys.argv) else '02.in')
part_1(strategy)
part_2(strategy)
