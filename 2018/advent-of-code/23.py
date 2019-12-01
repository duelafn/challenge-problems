#!/usr/bin/pypy
# -*- coding: utf-8 -*-

import argparse
import collections
import re

def dist(a, b):
    return sum(abs(a[i]-b[i]) for i in range(3))

class BBox(object):
    def __init__(self):
        self.xmin = self.xmax = None
        self.ymin = self.ymax = None
        self.zmin = self.zmax = None
    def update(self, pt):
        if self.xmin is None:
            self.xmin = self.xmax = pt[0]
            self.ymin = self.ymax = pt[1]
            self.zmin = self.zmax = pt[1]
        if pt[0] < self.xmin: self.xmin = pt[0]
        if pt[0] > self.xmax: self.xmax = pt[0]
        if pt[1] < self.ymin: self.ymin = pt[1]
        if pt[1] > self.ymax: self.ymax = pt[1]
        if pt[2] < self.zmin: self.zmin = pt[2]
        if pt[2] > self.zmax: self.zmax = pt[2]
        return self
    @property
    def width(self):
        return self.xmax-self.xmin+1
    @property
    def height(self):
        return self.ymax-self.ymin+1
    @property
    def depth(self):
        return self.zmax-self.zmin+1

    def contains(self, p):
        return (self.xmin <= p[0] <= self.xmax) and (self.ymin <= p[1] <= self.ymax) and (self.zmin <= p[2] <= self.zmax)

class Bot(object):
    # pos=<76659180,55463797,20890147>, r=80344142
    def __init__(self, i, pos, r):
        self.i = i
        self.pos = pos
        self.r = r

    def dist(self, b):
        return dist(self.pos, b.pos)

    def in_range(self, b):
        return dist(self.pos, b.pos) <= self.r

    def num_in_range(self, bots):
        return sum(1 for b in bots if dist(self.pos, b.pos) <= self.r)

    def show(self):
        print "Bot {} at {} with radius {}".format(self.i, self.pos, self.r)

DELTAS = (
    (-1,-1,-1), (-1,-1,0), (-1,-1,1), (-1,0,-1), (-1,0,0), (-1,0,1), (-1,1,-1), (-1,1,0), (-1,1,1),
    (0,-1,-1), (0,-1,0), (0,-1,1), (0,0,-1), (0,0,1), (0,1,-1), (0,1,0), (0,1,1),
    (1,-1,-1), (1,-1,0), (1,-1,1), (1,0,-1), (1,0,0), (1,0,1), (1,1,-1), (1,1,0), (1,1,1),
)

def search(bot, start=(0,0,0), mul=1, bbox=None):
    todo = collections.deque()
    seen = set()
    best_pos = start
    best_dist = sum(abs(x) for x in best_pos)
    best_count = sum(1 for b in bot if dist(best_pos, b.pos) <= b.r)

    todo.append(best_pos)
    while todo:
        pos = todo.popleft()
        if pos in seen: continue
        seen.add(pos)
        count = sum(1 for b in bot if dist(pos, b.pos) <= b.r)
        if count < best_count:
            continue
        elif count > best_count:
            best_pos = pos
            best_count = count
            best_dist = sum(abs(x) for x in pos)
            print "{} has better count ({}) at distance {}".format(pos, count, best_dist)
        elif sum(abs(x) for x in pos) < best_dist:
            best_pos = pos
            best_dist = sum(abs(x) for x in pos)
            print "{} has better distance ({}) at count {}".format(pos, best_dist, count)
        for dx, dy, dz in DELTAS:
            p2 = (pos[0]+mul*dx, pos[1]+mul*dy, pos[2]+mul*dz)
            if p2 not in seen and bbox.contains(p2):
                todo.append(p2)
    return (best_pos, best_dist, best_count)


def MAIN(argv):
    bot = []
    bbox = BBox()
    with open(argv.fname, 'r') as fh:
        for i, line in enumerate(fh):
            m = re.match(r'pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)', line)
            if m:
                bot.append( Bot(i, (int(m.group(1)), int(m.group(2)), int(m.group(3))), int(m.group(4))) )
                bbox.update(bot[-1].pos)
            else:
                raise Exception("Load error: " + line)

    print "Box of size {} x {} x {}".format(bbox.width, bbox.height, bbox.depth)
    bot.sort(key=lambda b: b.r)
    b0 = bot[-1]
    print "Strongest signal:"
    b0.show()
    print "Bots within range:", b0.num_in_range(bot)

    pos = (0,0,0)
    for exp in reversed(range(7)):
        step = 10**exp
        radius = 200 if pos == (0,0,0) else 25   # radius 10 is sufficient, but some overlap is fine
        bbox = BBox()
        for D in DELTAS:
            bbox.update(tuple(pos[i] + radius*step*D[i] for i in range(3)))
        pos, d, count = search(bot, pos, step, bbox)
        print "best position: {}  (distance {} from origin)  has {} bots in range  (at resolution {})".format(pos, d, count, step)



def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 23""")
    parser.add_argument('fname', type=str, nargs='?', default="23.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
