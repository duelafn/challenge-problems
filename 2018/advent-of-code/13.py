#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse


class Track(object):
    def __init__(self, sym, cart = None):
        self.symbol = sym
        self.cart = cart

class Behavior(object):
    def __init__(self):
        self.cycle = (1, 0, -1)  # left, forward, right
        self.step = -1

    def action(self):
        self.step = (1 + self.step) % len(self.cycle)
        return self.cycle[self.step]

class Board(object):
    def __init__(self):
        self.map = dict() # (y,x) => Track()
        self.carts = []
        self.to_remove = set()
        self.time = 0

    def load(self, fh):
        for y, line in enumerate(fh.readlines()):
            for x, ch in enumerate(line):
                if ch in "\\/|-+":
                    self.map[(y,x)] = Track(ch)
                elif ch in "<>":
                    c = Cart((y,x), 0 if ch == ">" else 2)
                    self.carts.append(c)
                    self.map[(y,x)] = Track("-", c)
                elif ch in "^v":
                    c = Cart((y,x), 1 if ch == "^" else 3)
                    self.carts.append(c)
                    self.map[(y,x)] = Track("|", c)
        return self

    def tick(self):
        self.time += 1
        self.carts.sort(key=lambda c: c.yx)
        for c in self.carts:
            self.step(c)
        if self.to_remove:
            self.carts = [ c for c in self.carts if c not in self.to_remove ]
            self.to_remove.clear()
        if 1 == len(self.carts):
            raise Exception("The cart standas alone t={} ad coordinate {},{}".format(self.time, *reversed(self.carts[0].yx)))

    def remove(self, *carts):
        self.to_remove = self.to_remove.union(carts)

    def step(self, c):
        t = self.map[c.yx]
        if not t.cart:
            return
        if t.symbol == '+':
            c.dir += c.behavior.action()
        elif t.symbol == '\\':
            if c.dir % 2: c.dir += 1
            else:         c.dir -= 1
        elif t.symbol == '/':
            if c.dir % 2: c.dir -= 1
            else:         c.dir += 1
        c.dir = c.dir % 4
        y, x = c.yx
        if   c.dir == 0: x += 1
        elif c.dir == 1: y -= 1
        elif c.dir == 2: x -= 1
        elif c.dir == 3: y += 1
        t2 = self.map[(y, x)]

        if t2.cart:
            print("Collision at t={} at coordinate {},{}".format(self.time, x, y))
            # Poof! remove the carts
            self.to_remove.add(c)
            self.to_remove.add(t2.cart)
            t.cart  = None
            t2.cart = None
            return

        c.yx = (y, x)
        t.cart = None
        t2.cart = c

class Cart(object):
    def __init__(self, yx, dir):
        self.yx = yx
        self.dir = dir
        self.behavior = Behavior()


def MAIN(argv):
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        map = Board().load(fh)

    while True:
        map.tick()


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 13""")
    parser.add_argument('fname', type=str, nargs='?', default="13.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
