#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse
import collections
import math
import numpy as np
import numpy.linalg


nan = float('nan')
class PatternFinder(object):
    def __init__(self, x, y, confidence=100, degree=2, cyclic=True, eps=1e-10):
        self.confidence = confidence
        self.degree = degree
        self.cyclic = cyclic
        self.solved = None
        self.eps = eps

    def start(self, x, y):
        self.start = x
        self.data = [y]
        self.deltas = [collections.deque(maxlen=2) for x in range(self.degree+1)]
        self.delta_len = [ 0 for x in range(self.degree+1) ]
        if self.cyclic:
            self.cycle = dict()
            self.cycle_mark = -1
            self.cycle_start = None
            self.cycle = { y: set((0,)) }

    def append(self, x, y):
        self.data.append((x,y))
        if self.degree:
            self.deltas[0].append(nan if 0 == self.data[-2][1] else y/self.data[-2][1])
            self.deltas[1].append(y-self.data[-2][1])
            for d in range(2, self.degree+1):
                self.deltas[d].append(self.deltas[d-1][1]-self.deltas[d-1][0])

            for d in range(self.degree+1):
                self.delta_len[d] = self.delta_len[d] + 1 if abs(self.deltas[d][0] - self.deltas[d][1]) < self.eps else 1
                if self.delta_len[d] >= self.confidence:
                    if d == 0:
                        A, B = self.data[-2:]
                        a = math.exp((math.log(B[1])-math.log(A[1]))/(B[0]-A[0]))
                        b = math.log(A[1]) - A[0]*math.log(a)
                        def power(x):
                            return b * a**x
                    else:
                        n = d+1
                        pts = self.data[-n:]
                        a = np.array([ [ p[0]**i for i in range(n) ] for p in pts ])
                        b = np.array([ p[1] for p in pts ])
                        coef = np.linalg.solve(a, b)
                        def poly(x):
                            return np.dot(coef, [ x**i for i in range(n) ])
                        self.solved = poly
                    break

        if self.cycle:
            if self.cycle_start is not None:
                pass

class World(object):
    def __init__(self):
        self.time = 0

    def step(self):
        self.time += 1
        for y, level in enumerate(self.data):
            for x, ch in enumerate(level):
                if ch == '.':
                    self.tmp[y][x] = '|' if self.count_adjacent(x, y, '|', 3) >= 3 else '.'
                elif ch == '|':
                    self.tmp[y][x] = '#' if self.count_adjacent(x, y, '#', 3) >= 3 else '|'
                elif ch == '#':
                    self.tmp[y][x] = '#' if self.count_adjacent(x, y, '|', 1) and self.count_adjacent(x, y, '#', 1) else '.'
        self.data, self.tmp = self.tmp, self.data

    def count_adjacent(self, x0, y0, ch, n=8):
        count = 0
        for dp in ( (-1,-1),(0,-1),(1,-1), (-1,0),(1,0), (-1,1),(0,1),(1,1) ):
            x = x0 + dp[0]
            y = y0 + dp[1]
            if 0 <= y < len(self.data):
                if 0 <= x < len(self.data[y]):
                    if self.data[y][x] == ch:
                        count += 1
                        if count >= n:
                            return count
        return count

    def value(self):
        score = { '.': 0, '#': 0, '|': 0 }
        for level in self.data:
            for ch in level:
                score[ch] += 1
        return score["#"] * score["|"]

    def load(self, fh):
        self.data = []
        for line in fh.readlines():
            self.data.append(list(line.strip()))
        self.tmp = list(list(x) for x in self.data)
        self.size = len(self.data)
        return self

    def show(self):
        for level in self.data:
            print("".join(level))
        print("")


def MAIN(argv):
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        world = World().load(fh)

    world.show()
    for i in range(10):
        world.step()
        world.show()
    print("Score:", world.value())

    for i in range(1000000000-10):
        world.step()
        print("Score (t={}): {}".format(world.time, world.value()))

def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 18""")
    parser.add_argument('fname', type=str, nargs='?', default="18.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())


"""
Score (t=1015): 190080
Score (t=1016): 192807
Score (t=1017): 194054
Score (t=1018): 197054
Score (t=1019): 199520
Score (t=1020): 199755
Score (t=1021): 200448
Score (t=1022): 198950
Score (t=1023): 195840
Score (t=1024): 193965
Score (t=1025): 193140
Score (t=1026): 191980
Score (t=1027): 191649
# Score (t=1028): 190820    # part 2
Score (t=1029): 190162
Score (t=1030): 190740
Score (t=1031): 187450
Score (t=1032): 186624
Score (t=1033): 186371
Score (t=1034): 187596
Score (t=1035): 187272
Score (t=1036): 187596
Score (t=1037): 189833
Score (t=1038): 189504
Score (t=1039): 189994
Score (t=1040): 190236
Score (t=1041): 190143
Score (t=1042): 187371
Score (t=1043): 190080
"""
