#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse
import numpy as np

class Claim(object):
    def __init__(self, line=None):
        self.id = self.left = self.top = self.width = self.height = None
        if line:
            id, _, xy, wh = line.split()
            self.id = id
            self.left,  self.top    = (int(x) for x in xy.rstrip(":").split(","))
            self.width, self.height = (int(x) for x in wh.split("x"))

    @property
    def right(self): return self.left + self.width
    @property
    def bottom(self): return self.top + self.height

    def draw(self, x, y, dtype=np.int8):
        m = np.zeros((x,y), dtype=dtype)
        m[self.left:self.right,self.top:self.bottom] = 1
        return m

    def __repr__(self): return "{} @ {},{}: {}x{}".format(self.id, self.left, self.top, self.width, self.height)

    def __lt__(self, b): return (self.left, self.right, self.bottom, self.top) < (b.left, b.right, b.bottom, b.top)
    def __eq__(self, b): return (self.left, self.right, self.bottom, self.top) == (b.left, b.right, b.bottom, b.top)


def MAIN(argv):
    claims = []
    W = H = 1
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        for line in fh.readlines():
            c = Claim(line)
            claims.append(c)
            if c.right  > W: W = c.right
            if c.bottom > H: H = c.bottom


    m = np.zeros((W,H), dtype=np.int8)
    for c in claims:
        c.drawn = c.draw(W, H)
        m += c.drawn
    print("Number of over-allocated inches:", np.sum(m>=2))

    tmp = np.zeros((W,H), dtype=np.int8)
    for c in claims:
        np.multiply(m, c.drawn, out=tmp)
        if np.all(np.equal(c.drawn, tmp)):
            print("Claim {} does not intersect with any other claim".format(c.id))



def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 03""")
    parser.add_argument('fname', type=str, default="03.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
