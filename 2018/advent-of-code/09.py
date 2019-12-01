#!/usr/bin/python3
# -*- coding: utf-8 -*-

import argparse

class Game(object):
    def __init__(self, players, marbles):
        self.players = players
        self.marbles = marbles  # we'll the 0 marble is the starting marble
        self.board = [ 0 ]
        self.score = [0] * players
        self.player = 0
        self.current = 0
        self.turn = 0

    def do_turn(self):
        self.turn += 1
        if 0 == (self.turn % 100_000):
            print(f'{self.turn}: {max(self.score)}')
        if self.turn % 23:
            self.current = (self.current + 2)
            if self.current == len(self.board):
                self.board.append(self.turn)
            else:
                self.current = self.current % len(self.board)
                self.board.insert(self.current, self.turn)
        else:
            self.current = (self.current + -7) % len(self.board)
            self.score[(self.turn-1)%self.players] += self.turn + self.board.pop(self.current)

    def play(self):
        for i in range(self.marbles):
            self.do_turn()

def MAIN(argv):
    game = Game(argv.players, argv.marbles)
    game.play()
    print(max(game.score))


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 09""")
    parser.add_argument('players', type=int, help='players')
    parser.add_argument('marbles', type=int, help='marbles')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
