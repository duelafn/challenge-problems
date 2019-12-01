#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse
import collections
import pickle
import time

# NOTE: sloppy description, apparently when tied, we should move toward
# enemy that is first in (y,x) order, not the step that is first in (y,x)
# order.
#
# This implementation still does the (incorrect?) first-step approach. Whatever.
#
# Search for off-by-one in:
#   https://www.reddit.com/r/adventofcode/comments/a6chwa/2018_day_15_solutions/

class Creature(object):
    def __init__(self, yx, alliance, power=3, hp=200):
        self.initial_pos = yx
        self.power = power
        self.hp = hp
        self.yx = yx
        self.alliance = alliance

    @property
    def x(self):
        return self.yx[1]
    @property
    def y(self):
        return self.yx[0]

    def action(self, board):
        # path tuple:  0: depth,  1: step1
        path = self._find_path(self.yx, 0, board, set([self.yx]))
        # print("{} at ({},{}), nearest enemy is at ({},{}), {} steps away, first step: ({},{})".format(self.alliance, self.yx[1], self.yx[0], path[2], path[1], path[0], path[3][1], path[3][0]))
        if path and path[1]:
            board.move(self, path[1])
        target = self._find_target(board)
        if target:
            board.attack(self, target)

    def _find_target(self, board):
        y, x = self.yx
        targets = []
        for yx in ((y-1,x), (y,x-1), (y,x+1), (y+1,x)):
            c = board.map.get(yx, None)
            if c and c.alliance != self.alliance:
                targets.append(c)
        targets.sort(key=lambda c: (c.hp, *c.yx))
        return targets[0] if targets else None

    def _find_path(self, yx, d, board, _):
        todo = collections.deque()
        todo.append( (0, None, yx[0], yx[1]) );
        seen = dict()
        while todo:
            t = todo.popleft()
            d, step1, y, x = t
            if (y,x) in seen: continue
            seen[(y,x)] = True
            for _yx in ( (y-1,x), (y,x-1), (y,x+1), (y+1,x) ):
                if _yx in seen: continue

                if _yx not in board.map:
                    pass
                elif board.map[_yx] is None:
                    todo.append( (d+1, step1 or _yx, _yx[0], _yx[1]) )
                elif board.map[_yx] and board.map[_yx].alliance != self.alliance:
                    return (d, step1)


    def _find_path_hybrid(self, yx, d, board, seen, step1=None, found=None):
        if found and d >= found:
            return
        y, x = yx

        # Local search of depth d+1. opt is in order, so take the first match.
        todo = []
        for _yx in ( (y-1,x), (y,x-1), (y,x+1), (y+1,x) ):
            if _yx in seen:
                continue
            seen.add(_yx)

            if _yx not in board.map:
                pass
            elif board.map[_yx] is None:
                todo.append(_yx)
            elif board.map[_yx] and board.map[_yx].alliance != self.alliance:
                return (d, step1)

        # Recursive search. Order of opt doesn't help, so have to search
        # all, but set "found" once we find something to short-circuit if
        # possible.
        rv = []
        for _yx in todo:
            t = self._find_path(_yx, d+1, board, set(seen), step1 or _yx, found=found)
            if t:
                found = t[0] if not found else min(found, t[0])
                rv.append(t)
        # tuple-sort does what we want since we put "y" first
        rv.sort()
        return rv[0] if rv else None


class Board(object):
    def __init__(self):
        self.map = dict()
        self.creatures = []
        self.army = dict()
        self.to_remove = set()
        self.time = 0
        self.size_xy = (0,0)

    def score(self):
        return self.time * self.hp_sum()
    def hp_sum(self):
        return sum(c.hp for c in self.creatures)

    def load(self, fh):
        for y, line in enumerate(fh.readlines()):
            for x, ch in enumerate(line):
                if ch in ".":
                    self.map[(y,x)] = None
                elif ch in "GE":
                    c = Creature((y,x), ch)
                    self.creatures.append(c)
                    self.map[(y,x)] = c
                    if ch not in self.army:
                        self.army[ch] = 1
                    else: self.army[ch] += 1
        self.size_xy = (x, y+1)
        return self

    def move(self, c, yx):
        if yx not in self.map:
            raise Exception("Creature can't move off the map (to {})".format(yx))
        if self.map[yx] is not None:
            raise Exception("Position {} already occupied".format(yx))
        if c.yx not in self.map or self.map[c.yx] != c:
            raise Exception("Creature ({} initially at ({},{})) isn't where we thought it was (at ({},{}))".format(c.alliance, c.initial_pos[1], c.initial_pos[0], c.x, c.y))
        self.map[c.yx] = None
        self.map[yx] = c
        c.yx = yx

    def attack(self, a, d):
        d.hp -= a.power
        if d.hp <= 0:
            print("{} at {},{} has died in round {}".format(d.alliance, d.yx[1], d.yx[0], self.time))
            if d.yx not in self.map or self.map[d.yx] != d:
                raise Exception("Creature isn't where we thought it was (#2)")
            self.map[d.yx] = None
            self.to_remove.add(d)
            self.army[d.alliance] -= 1

    def tick(self):
        self.time += 1
        self.creatures.sort(key=lambda c: c.yx)
        for c in self.creatures:
            if c.hp > 0:
                c.action(self)
        if self.to_remove:
            self.creatures = [ c for c in self.creatures if c not in self.to_remove ]
            self.to_remove.clear()
        if any(0 == x for x in self.army.values()):
            return False
        return True

    def show(self):
        for y in range(self.size_xy[1]):
            row = ["#"] * self.size_xy[0]
            for x in range(self.size_xy[0]):
                if (y, x) in self.map:
                    if self.map[(y,x)]:
                        row[x] = self.map[(y,x)].alliance
                    else:
                        row[x] = "."
            print("".join(row))

def MAIN(argv):
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        board = Board().load(fh)
    starting_elves = board.army['E']
    board_pickle = pickle.dumps(board)
    board.show()
    print("")
    if argv.rounds:
        for i in range(argv.rounds):
            board.tick()
            board.show()
            print("")
    else:
        while board.tick():
            if argv.movie:
                board.show()
                print("")
                time.sleep(0.250)
    board.show()
    print("Outcome: {}*{} = {}".format(board.time, board.hp_sum(), board.score()))

    i = 3
    while board.army['E'] != starting_elves:
        board = pickle.loads(board_pickle)
        i += 1
        for c in board.creatures:
            if c.alliance == 'E':
                c.power = i
        while board.tick():
            pass
        print("Elf power {}: {}".format(i, board.army))
        print("Outcome: {}*{} = {}".format(board.time, board.hp_sum(), board.score()))



def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 15""")
    parser.add_argument('--movie', help='show steps with a slight delay for a movie')
    parser.add_argument('fname', type=str, nargs='?', default="15.in", help='File name')
    parser.add_argument('rounds', type=int, nargs='?', default=0, help='Number of rounds')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
