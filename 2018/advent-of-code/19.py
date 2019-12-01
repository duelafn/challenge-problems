#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse

class VM(object):
    def __init__(self, ip):
        self.ip = ip
        self.reg = [0, 0, 0, 0, 0, 0]
        self.reg[self.ip] -= 1

    def run(self, prog):
        while -1 <= self.reg[self.ip] < len(prog)-1:
            self.reg[self.ip] += 1
            op, a, b, c = prog[self.reg[self.ip]]
            VM.__dict__[op](self.reg, a, b, c)
            print(self.reg)
        return self.reg

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


def MAIN(argv):
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        prog = []
        for line in fh.readlines():
            if line.startswith("#ip "):
                vm = VM(int(line[4]))
            else:
                l = line.split()
                prog.append([ l[0], int(l[1]), int(l[2]), int(l[3]) ])

#     print("Part 1:", vm.run(prog))

    vm.reg = [1, 0, 0, 0, 0, 0]
    vm.reg[vm.ip] -= 1
    print("Part 2:", vm.run(prog))

def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 19""")
    parser.add_argument('fname', type=str, nargs='?', default="19.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
