#!/usr/bin/pypy
# -*- coding: utf-8 -*-

import argparse
import collections

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

State = collections.namedtuple("State", "time pos tool".split())

class Map(object):
    TYPE = { 0: '.', 1: '=', 2: '|' }
    RISK = { '.': 0, '=': 1, '|': 2 }
    TOOLS = { '.': ("c", "t"), '=': ("c", "n"), '|': ("t", "n") }

    def __init__(self):
        self.bbox = BBox()
        self.data  = { }
        self.level = { }

    def geolevel(self, x, y):
        rv = self.level.get((x,y), None)
        if rv is not None: return rv
        self.geotype(x, y)
        return self.level[(x,y)]

    def geotype(self, x, y):
        rv = self.data.get((x,y), None)
        if rv is not None: return rv

        self.bbox.update((x,y))
        if y == 0:
            index = x * 16807
        elif x == 0:
            index = y * 48271
        else:
            index = self.geolevel(x-1, y) * self.geolevel(x, y-1)

        self.level[(x,y)] = (index + self.depth) % 20183
        self.data[(x,y)] = rv = self.TYPE[self.level[(x,y)] % 3]
        return rv

    def georisk(self, x, y):
        return self.RISK[self.geotype(x,y)]

    def risk(self, bbox):
        risk = 0
        for y in range(bbox.ymin, bbox.ymax+1):
            risk += sum(self.georisk(x, y) for x in range(bbox.xmin, bbox.xmax+1))
        return risk

    def route(self, a, b):
        todo = collections.deque()
        seen = dict()
        best = None
        dirs = ( (0,-1), (-1,0), (1,0), (0,1) )

        todo.append(State(0, a, "t"))
        while todo:
            s = todo.popleft()
            if (s.pos, s.tool) in seen and seen[(s.pos, s.tool)].time <= s.time:
                continue
            seen[(s.pos, s.tool)] = s

            if best is not None and s.time > best.time:
                continue

            if s.pos == b and s.tool == 't':
                if best is None or s.time < best.time:
                    best = s
                continue

            for dx, dy in dirs:
                p2 = (s.pos[0]+dx, s.pos[1]+dy)
                if p2[0] < 0 or p2[1] < 0:
                    continue
                typ = self.geotype(*p2)
                if s.tool in self.TOOLS[typ]:
                    todo.append(State(s.time+1, p2, s.tool))

            for t in self.TOOLS[self.geotype(*s.pos)]:
                if t != s.tool:
                    todo.append(State(s.time+7, s.pos, t))
        return best

    def load(self, fh):
        for line in fh.readlines():
            if line.startswith("depth"):
                self.depth = int(line[7:].strip())
            elif line.startswith("target"):
                self.target = tuple(int(x) for x in line[8:].strip().split(','))
        self.level[self.target] = self.depth % 20183
        self.data[self.target] = self.TYPE[self.level[self.target] % 3]
        return self

    def show(self):
        for y in range(self.bbox.ymin, self.bbox.ymax+1):
            print "".join(self.data.get((x,y), "?") for x in range(self.bbox.xmin, self.bbox.xmax+1))
        print


def MAIN(argv):
    with open(argv.fname, 'r') as fh:
        map = Map().load(fh)

    print "Part 1 Risk:", map.risk(BBox().update((0,0)).update(map.target))

    print "Part 2 Route:", map.route((0,0), map.target)

def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 22""")
    parser.add_argument('fname', type=str, nargs='?', default="22.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
