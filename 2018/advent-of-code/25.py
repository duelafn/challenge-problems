#!/usr/bin/pypy
# -*- coding: utf-8 -*-

import argparse

class Thing(object):
    def __init__(self):
        pass

    def load(self, fh):
        for y, line in enumerate(fh.readlines()):
            pass
        return self

def dist(a, b):
    return sum(abs(a[i]-b[i]) for i in range(4))

def cluster(point):
    d = dict()
    c = []
    for p in point:
        added = False
        for q in d.keys():
            if dist(p, q) <= 3:
                if added:
                    if d[p] != d[q]:
                        i, j = (d[p], d[q]) if d[p] < d[q] else (d[q], d[p])
                        c[i].extend(c[j])
                        for r in c[j]:
                            d[r] = i
                        c[j] = None
#                         print "{} merges constellations {} and {}".format(p, i, j)
                else:
                    d[p] = d[q]
                    c[d[p]].append(p)
#                     print "{} joins constellation {}".format(p, d[q])
                    added = True
        if not added:
            i = len(c)
            c.append([p])
            d[p] = i
#             print "{} forms its own constellation, {}".format(p, i)
    return [ l for l in c if l ]


def MAIN(argv):
    point = []
    with open(argv.fname, 'r') as fh:
        for line in fh:
            point.append(tuple([ int(x) for x in line.strip().split(",") ]))

    clusters = cluster(point)
    print "{} constellations".format(len(clusters))
    if len(point) < 20:
        print clusters


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 25""")
    parser.add_argument('fname', type=str, nargs='?', default="25.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
