#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse
import re

class BBox(object):
    def __init__(self):
        self.xmin = self.xmax = None
        self.ymin = self.ymax = None
    def update(self, pt):
        if self.xmin is None:
            self.xmin = self.xmax = pt[0]
            self.ymin = self.ymax = pt[1]
        if pt[0] < self.xmin: self.xmin = pt[0]
        if pt[0] > self.xmax: self.xmax = pt[0]
        if pt[1] < self.ymin: self.ymin = pt[1]
        if pt[1] > self.ymax: self.ymax = pt[1]
        return self
    @property
    def width(self):
        return self.xmax-self.xmin+1
    @property
    def height(self):
        return self.ymax-self.ymin+1

class Dirt(object):
    def __init__(self):
        self.bbox = BBox()

    def load(self, fh):
        v = []
        h = []
        for line in fh.readlines():
            m = re.match(r'([xy])=(\d+), [xy]=(\d+)\.\.(\d+)', line)
            if m:
                t = (int(m.group(2)), int(m.group(3)), int(m.group(4)))
                if m.group(1) == 'x':
                    v.append(t)
                    self.bbox.update((t[0], t[1]))
                    self.bbox.update((t[0], t[2]))
                else:
                    h.append(t)
                    self.bbox.update((t[1], t[0]))
                    self.bbox.update((t[2], t[0]))

        # X is infinite, allow overflow of clay touching the boundary:
        self.bbox.xmin -= 1
        self.bbox.xmax += 1

        width = self.bbox.width
        self.data = [ [None]*width for i in range(self.bbox.height) ]
        for t in v:
            x = t[0]-self.bbox.xmin
            for y in range(t[1]-self.bbox.ymin, t[2]+1-self.bbox.ymin):
                self.data[y][x] = "#"
        for t in h:
            y = t[0]-self.bbox.ymin
            for x in range(t[1]-self.bbox.xmin, t[2]+1-self.bbox.xmin):
                self.data[y][x] = "#"
        return self

    def show(self):
        for level in self.data:
            print("".join(x or "." for x in level))
        print("")

    def fill(self, init=None):
        if init is None:
            init = (0, 500-self.bbox.xmin)
        source = self._drip(init)
        while source:
            y, x = source.pop()
            if self.data[y][x] == '~':
                continue
#             elif self.data[y][x]:
#                 raise Exception("Premature marking!")

            if y == len(self.data)-1:
                self.data[y][x] = '|'
            else:
                below = self.data[y+1][x]
                if below is None:
                    raise Exception("Didn't expect that")
                if below == "|":
                    self.data[y][x] = '|'
                else:
                    l = len(source)
                    for pt in self._flood((y,x)):
                        source.extend(self._drip(pt))
                    if len(source) > l:
                        source.sort()

    def count_wet(self):
        count = 0
        for level in self.data:
            count += sum(1 for x in level if x in ('~', '|'))
        return count

    def count_flooded(self):
        count = 0
        for level in self.data:
            count += sum(1 for x in level if x == '~')
        return count

    def _drip(self, pt):
        y, x = pt
        rv = [ pt ]
        while y < len(self.data)-1:
            y += 1
            if self.data[y][x]:
                return rv
            rv.append( (y, x) )
        return rv

    def _flood(self, pt):
        drip = []
        y, xmax = pt
        while self.data[y][xmax+1] in (None, '|'):
            xmax += 1
            if self.data[y+1][xmax] in (None, '|'):
                shape = '|'
                drip.append( (y, xmax) )
                xmax -= 1    # Don't mark the drip point
                break

        y, xmin = pt
        while self.data[y][xmin-1] in (None, '|'):
            xmin -= 1
            if self.data[y+1][xmin] in (None, '|'):
                shape = '|'
                drip.append( (y, xmin) )
                xmin += 1    # Don't mark the drip point
                break

        for x in range(xmin, xmax+1):
            self.data[y][x] = '|' if drip else '~'
        return drip


def MAIN(argv):
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        dirt = Dirt().load(fh)
    if dirt.bbox.width < 200:
        dirt.show()
    dirt.fill()
    if dirt.bbox.width < 200:
        dirt.show()
    print("Wet zones:", dirt.count_wet())
    print("Flood zones:", dirt.count_flooded())


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 17""")
    parser.add_argument('fname', type=str, nargs='?', default="17.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
