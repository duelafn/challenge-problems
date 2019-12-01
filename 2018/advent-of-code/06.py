#!/usr/bin/python3
# -*- coding: utf-8 -*-

import math
import argparse

class BBox(object):
    def __init__(self, pt=(0,0)):
        self.xmin = pt[0]
        self.xmax = pt[0]
        self.ymin = pt[1]
        self.ymax = pt[1]

    def update(self, pt):
        if pt[0] < self.xmin: self.xmin = pt[0]
        if pt[0] > self.xmax: self.xmax = pt[0]
        if pt[1] < self.ymin: self.ymin = pt[1]
        if pt[1] > self.ymax: self.ymax = pt[1]

    def grow(self, px):
        self.xmin -= px
        self.xmax += px
        self.ymin -= px
        self.ymax += px

    def boundary(self):
        for x in range(self.xmin, self.xmax+1):
            yield (x, self.ymin)
            yield (x, self.ymax)
        for y in range(self.ymin+1, self.ymax):
            yield (self.xmax, y)
            yield (self.xmin, y)

    def interior(self):
        for x in range(self.xmin+1, self.xmax):
            for y in range(self.ymin+1, self.ymax):
                yield (x, y)


def distance(a, b):
    return abs(a[0]-b[0]) + abs(a[1]-b[1])

def cloud_dist(points, b):
    d = 0
    for a in points:
        d += abs(a[0]-b[0]) + abs(a[1]-b[1])
    return d

def nearest(pt, points):
    d = math.inf
    for i, p in enumerate(points):
        _d = distance(pt, p)
        if _d < d:
            idx = [ i ]
            d = _d
        elif _d == d:
            idx.append(i)
    return d, idx

def load(fname):
    points = []
    bbox = None
    with open(fname, 'r', encoding='UTF-8') as fh:
        for line in fh.readlines():
            points.append([ int(v.strip()) for v in line.split(',') ])
            if bbox is None:
                bbox = BBox(points[-1])
            else:
                bbox.update(points[-1])
    return points, bbox

def MAIN(argv):
    points, bbox = load(argv.fname)
    bbox.grow(5)

    if 1 == argv.part:
        size = [ 0 ] * len(points)
        for pt in bbox.boundary():
            d, idx = nearest(pt, points)
            if 1 == len(idx):
                size[idx[0]] = -math.inf

        max_area = 0
        indices = []
        for pt in bbox.interior():
            d, idx = nearest(pt, points)
            if 1 == len(idx):
                size[idx[0]] += 1
                if size[idx[0]] > max_area:
                    indices = [ idx[0] ]
                    max_area = size[idx[0]]
                elif size[idx[0]] == max_area:
                    indices.append(idx[0])

        print("Index {} has the largest bounded area: {}".format(indices, max_area))

    else:
        size = 0
        for pt in bbox.interior():
            d = cloud_dist(points, pt)
            if d < argv.distance:
                size += 1
        grow = True
        while grow:
            grow = False
            for pt in bbox.boundary():
                d = cloud_dist(points, pt)
                if d < argv.distance:
                    size += 1
                    grow = True
        print("Safe zone has size {}".format(size))


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 06""")
    parser.add_argument('part', nargs=1, type=int, help='part number')
    parser.add_argument('fname', nargs='?', type=str, default="06.in", help='File name')
    parser.add_argument('--distance', nargs='?', type=int, default=10000, help='part number')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
