#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse
import re
import sys, tty, termios

def getch():
    fd = sys.stdin.fileno()
    old_settings = termios.tcgetattr(fd)
    try:
        tty.setraw(sys.stdin.fileno())
        ch = sys.stdin.read(1)
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
    return ch

class BBox(object):
    def __init__(self, pt=(0,0)):
        self.xmin = pt[0]
        self.xmax = pt[0]
        self.ymin = pt[1]
        self.ymax = pt[1]

    def bbox(self):
        return (self.xmin, self.xmax, self.ymin, self.ymax)

    def update(self, pt):
        if pt[0] < self.xmin: self.xmin = pt[0]
        if pt[0] > self.xmax: self.xmax = pt[0]
        if pt[1] < self.ymin: self.ymin = pt[1]
        if pt[1] > self.ymax: self.ymax = pt[1]

    def diameter(self):
        return max(self.xmax-self.xmin, self.ymax-self.ymin)

    def width(self):
        return self.xmax-self.xmin

    def height(self):
        return self.ymax-self.ymin

class Point(object):
    def __init__(self, pos, vel):
        self.pos = pos
        self.vel = vel

    def step(self, n=1):
        self.pos = (self.pos[0] + n*self.vel[0], self.pos[1] + n*self.vel[1])

class Sky(object):
    def __init__(self):
        self.stars = []
        self.t = 0

    def step(self, n=1):
        self.t += n
        self.bbox = None
        for s in self.stars:
            s.step(n)
            if self.bbox:
                self.bbox.update(s.pos)
            else:
                self.bbox = BBox(s.pos)

    def show(self):
        width, height = self.bbox.width(), self.bbox.height()
        m = []
        for _ in range(height+1):
            m.append([" "] * (width+1))
        for s in self.stars:
            x, y = s.pos[0] - self.bbox.xmin, s.pos[1] - self.bbox.ymin
            m[y][x] = "#"
        for row in m:
            print("".join(row))

    def load(self, fname):
        self.bbox = None
        with open(fname, 'r', encoding='UTF-8') as fh:
            for line in fh.readlines():
                # position=< 20316, -30055> velocity=<-2,  3>
                m = re.search(r'<\s*(-?\d+),\s*(-?\d+)>.*?<\s*(-?\d+),\s*(-?\d+)>', line)
                s = Point((int(m.group(1)), int(m.group(2))), (int(m.group(3)), int(m.group(4))))
                self.stars.append(s)
                if self.bbox:
                    self.bbox.update(s.pos)
                else:
                    self.bbox = BBox(s.pos)

def MAIN(argv):
    sky = Sky()
    sky.load(argv.fname)
    diam = sky.bbox.diameter() + 1
    while diam > sky.bbox.diameter():
        diam = sky.bbox.diameter()
        sky.step()
    sky.step(-1)

    while True:
        print("time =", sky.t)
        sky.show()
        c = getch()
        if c == "+":
            sky.step(1)
        elif c == "-":
            sky.step(-1)
        else:
            break

def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 10""")
    parser.add_argument('fname', type=str, nargs='?', default="10.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
