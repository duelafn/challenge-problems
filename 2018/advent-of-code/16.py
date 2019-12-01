#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse
import re


class VM(object):
    CMD = '''
    addr addi mulr muli banr bani borr bori
    setr seti gtir gtri gtrr eqir eqri eqrr
    '''.split()

    def __init__(self):
        self.trans = dict()
        self.can = dict()
        for i in range(16):
            self.can[i] = set(self.CMD)

    def run(self, prog, reg=None):
        if reg is None:
            reg = [0, 0, 0, 0]
        for cmd in prog:
            self.apply(reg, *cmd)
        return reg

    def apply(self, R, op, a, b, c):
        if op in self.trans:
            op = self.trans[op]
        if op in VM.__dict__:
            VM.__dict__[op](R, a, b, c)
            return R
        else:
            raise Exception("Unknown operation '{}'".format(op))

    def op_assert(self, op, bef, aft):
        can = set()
        for name in self.can[op[0]]:
            R = self.apply(list(bef), name, op[1], op[2], op[3])
            if R == aft:
                can.add(name)
        self.can[op[0]] = can
        return can

    def op_solve(self):
        repeat = True
        while repeat:
            repeat = False
            for op, can in self.can.items():
                if 1 == len(can):
                    known = can.pop()
                    self.trans[op] = known
                    repeat = True
                    for s in self.can.values():
                        s.discard(known)

    def addr(R, a, b, c): R[c] = R[a] + R[b]
    def addi(R, a, b, c): R[c] = R[a] + b

    def mulr(R, a, b, c): R[c] = R[a] * R[b]
    def muli(R, a, b, c): R[c] = R[a] * b

    def banr(R, a, b, c): R[c] = R[a] & R[b]
    def bani(R, a, b, c): R[c] = R[a] & b

    def borr(R, a, b, c): R[c] = R[a] | R[b]
    def bori(R, a, b, c): R[c] = R[a] | b

    def setr(R, a, b, c): R[c] = R[a]
    def seti(R, a, b, c): R[c] = a

    def gtir(R, a, b, c): R[c] = 1 if a > R[b] else 0
    def gtri(R, a, b, c): R[c] = 1 if R[a] > b else 0
    def gtrr(R, a, b, c): R[c] = 1 if R[a] > R[b] else 0

    def eqir(R, a, b, c): R[c] = 1 if a == R[b] else 0
    def eqri(R, a, b, c): R[c] = 1 if R[a] == b else 0
    def eqrr(R, a, b, c): R[c] = 1 if R[a] == R[b] else 0

class Program(object):
    def __init__(self, lines):
        self.lines = lines

class Test(object):
    def __init__(self, cmd, before, after):
        self.cmd = cmd
        self.bef = before
        self.aft = after

def vec(s):
    return [ int(x) for x in re.split(r'\D+', s) if re.match(r'\d+', x) ]

def MAIN(argv):
    tests = []
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        for b in re.split(r'\n\n+', fh.read()):
            lines = b.split("\n")
            if 3 == len(lines):
                tests.append(Test(vec(lines[1]), vec(lines[0]), vec(lines[2])))
            else:
                program = Program([ vec(x) for x in lines if x ])

    part1 = 0
    vm = VM()
    for t in tests:
        vm.op_assert(t.cmd, t.bef, t.aft)
        tmp = VM()
        can = tmp.op_assert(t.cmd, t.bef, t.aft)
        if len(can) >= 3:
            part1 += 1

    print("Part 1: {}".format(part1))

    vm.op_solve()
    R = vm.run(program.lines)
    print("Part 2: {}".format(R[0]))


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 16""")
    parser.add_argument('fname', type=str, nargs='?', default="16.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
