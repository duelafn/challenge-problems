#!/usr/bin/python3
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

class Map(object):
    def __init__(self):
        self.bbox = BBox()
        self.data = dict()

    def load(self, fh):
        txt = fh.read()
        vec = dict(N=(0,-1), S=(0,1), E=(1,0), W=(-1,0))
        stack = [ [[0,0]] ]
        self.data[(0,0)] = 'X'
        for ch in txt:
            if ch in 'NSEW':
                dx, dy = vec[ch]
                for p in stack[-1]:
                    x, y = p
                    p[:] = x+2*dx, y+2*dy
                    self.bbox.update((x+2*dx, y+2*dy))
                    self.data[(x+dx, y+dy)] = '|' if dy == 0 else '-'
                    self.data[(x+2*dx, y+2*dy)] = '.'
            elif ch == '(':
                current = stack.pop()
                stack.append([  ])  # Future positions (after "()" closes)
                stack.append([ list(x) for x in current ])  # Saved pos at "("
                stack.append([ list(x) for x in current ])  # Current updatable position
            elif ch == ')':
                current = stack.pop()
                # Now: [-1] == saved pos list; [-2] == future pos list
                stack.pop()  # No longer need saved state
                stack[-1].extend(current)
                stack[-1] = [ [x,y] for x,y in set((x,y) for x,y in stack[-1]) ]
            elif ch == '|':
                current = stack.pop()
                # Now: [-1] == saved pos list; [-2] == future pos list
                stack[-2].extend(current)
                stack.append([ list(x) for x in stack[-1] ])  # Current updatable position
        return self

    def shortest_paths(self, pos=(0,0)):
        self.diameter = 0
        pos = tuple([ pos ])
        self.paths = paths = { pos: [pos] }
        dirs = [ (0,-1), (0,1), (1,0), (-1,0) ]
        todo = collections.deque([ pos ])
        while todo:
            path = todo.popleft()
            x, y = path[-1]
            for d in dirs:
                x1, y1 = x+d[0], y+d[1]
                if self.data.get((x1,y1), "#") != "#":
                    x2, y2 = x+2*d[0], y+2*d[1]
                    if (x2, y2) not in paths:
                        self.diameter = len(path)
                        nxt = path + ((x2, y2),)
                        paths[(x2, y2)] = nxt
                        todo.append(nxt)

    def show(self):
        for y in range(self.bbox.ymin-1, self.bbox.ymax+2):
            print("".join(self.data.get((x,y), "#") for x in range(self.bbox.xmin-1, self.bbox.xmax+2)))

def MAIN(argv):
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        map = Map().load(fh)
    map.show()

    map.shortest_paths()
    print("Map diameter:", map.diameter)

    print("At least 1000 away:", sum(1 for p in map.paths.values() if len(p) > 1000))


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 20""")
    parser.add_argument('fname', type=str, nargs='?', default="20.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
