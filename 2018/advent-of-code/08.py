#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse

class Memory(object):
    def __init__(self, data):
        self.data = data
        self.idx = 0

    def read(self):
        self.idx += 1
        return int(self.data[self.idx-1])


class Node(object):
    def __init__(self, mem):
        self.child = []
        self.meta  = []

        num_c = mem.read()
        num_m = mem.read()
        for i in range(num_c):
            self.child.append(Node(mem))
        for i in range(num_m):
            self.meta.append(mem.read())

    def metasum(self):
        return sum(self.meta) + sum(x.metasum() for x in self.child)

    def value(self):
        if self.child:
            val = 0
            for idx in self.meta:
                if 0 < idx <= len(self.child):
                    val += self.child[idx-1].value()
            return val
        else:
            return sum(self.meta)


def MAIN(argv):
    with open(argv.fname, 'r', encoding='UTF-8') as fh:
        mem = Memory(fh.read().strip().split())
    tree = Node(mem)

    print("Metasum:", tree.metasum())
    print("Value:", tree.value())


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 08""")
    parser.add_argument('fname', type=str, nargs='?', default="08.in", help='File name')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
