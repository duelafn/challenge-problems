#!/usr/bin/pypy
# -*- coding: utf-8 -*-

import argparse
import pickle
import re

def sort_target_selection_order(a):
    return (-a.effective_power, -a.initiative)

class Unit(object):
    def __init__(self):
        self.hp = None
        self.immune = set()
        self.weak = set()
        self._power = None
        self.attack_type = None
        self.initiative = None
        self.boost = 0

    def load(self, line):
        self.hp = int(re.search(r'(\d+) hit points', line).group(1))
        m = re.search(r'immune to ([\w ,]+)', line)
        if m: self.immune = set(x.strip() for x in m.group(1).split(','))
        m = re.search(r'weak to ([\w ,]+)', line)
        if m: self.weak = set(x.strip() for x in m.group(1).split(','))
        m = re.search(r'attack that does (\d+) (\w+) damage at initiative (\d+)', line)
        self.power = int(m.group(1))
        self.attack_type = m.group(2)
        self.initiative = int(m.group(3))
        return self

    @property
    def power(self):
        return self.boost + self._power
    @power.setter
    def power(self, val):
        self._power = val

    def damage_to(self, target):
        if self.attack_type in target.immune:
            return 0
        if self.attack_type in target.weak:
            return 2 * self.power
        return self.power

    def receive_damage(self, damage):
        self.hp -= damage
        if self.hp < 0:
            self.hp = 0


class Army(Unit):
    def __init__(self, id, team):
        super(Army,self).__init__()
        self.id = id
        self.team = team
        self.units = None

    @property
    def effective_power(self):
        return self.units * self.power

    def damage_to(self, target):
#         print " : {} {} deals {} * {} = {} damage to {} {}".format(self.team, self.id, self.units, super(Army,self).damage_to(target), self.units*super(Army,self).damage_to(target), target.team, target.id)
        return self.units * super(Army,self).damage_to(target)

    def select_target(self, targets):
        best = (0,0,0,None)
        for t in targets:
            this = (self.damage_to(t), t.effective_power, t.initiative, t)
#             print "  : {} {} considering target {} {}: score {}   ; best so far {}".format(self.team, self.id, t.team, t.id, this[0:3], best[0:3])
            if this > best:
                best = this
        return best[3] if best[0] > 0 else None

    def receive_damage(self, damage):
        units = int(damage / self.hp)
        if units > self.units:
            units = self.units
        self.units -= units
        return units

    def load(self, line):
        super(Army,self).load(line)
        self.units = int(re.search(r'(\d+) units', line).group(1))
        return self

    def show(self):
        print "{} {} has {} units remaining".format(self.team, self.id, self.units)


def fight(armies):
    # Defending groups can only be chosen as a target by one attacking group
    attackers = sorted((a for a in armies if a.units > 0), key=sort_target_selection_order)
    defenders = set(d for d in armies if d.units > 0)
    fights = []
#     for a in attackers:
#         for t in defenders:
#             if a.team != t.team:
#                 print "{} {} targets {} {}  (attacker key={} ; target key={})".format(
#                     a.team, a.id, t.team, t.id,
#                     (a.effective_power, a.initiative),
#                     (a.damage_to(t), t.effective_power, t.initiative),
#                 )
#     print "==="
    for a in attackers:
        t = a.select_target(d for d in defenders if d.team != a.team)
#         print "{} {} targets {} {}  (attacker key={} ; target key={})".format(
#             a.team, a.id, t.team, t.id,
#             (a.effective_power, a.initiative),
#             (a.damage_to(t), t.effective_power, t.initiative),
#         )
        if t is not None:
            fights.append((a,t))
            defenders.remove(t)
#     print '---'
    fights.sort(key=lambda f: -f[0].initiative)
    did_something = False
    for f in fights:
        a, d = f
        damage = a.damage_to(d)
        if damage:
            deaths = d.receive_damage(damage)
            if deaths > 0:
                did_something = True
#             print "{} {} attacks {} {} dealing {} damage killing {} units".format(a.team, a.id, d.team, d.id, damage, deaths)
    if not did_something:
#         for a in armies: a.show()
        raise Exception("Failed to kill anyone!")
#     print '---'

def try_boost(n, armies):
    for a in armies:
        if a.team == 'immune':
            a.boost = n
    try:
        while True:
            fight(armies)
            alive = set(a.team for a in armies if a.units > 0)
            if 1 == len(alive): break
        return alive.pop()
    except Exception as err:
        return None

def MAIN(argv):
    armies = []
    with open(argv.fname, 'r') as fh:
        for line in fh:
            line = line.strip()
            if line == 'Immune System:':
                team = 'immune'
                group = 0
            elif line == 'Infection:':
                team = 'infection'
                group = 0
            elif line:
                group += 1
                armies.append(Army(group, team).load(line))
#                 a = armies[-1]
#                 print "{} {} weak: '{}'  immune: '{}'  damage: '{}'".format(a.team, a.id, a.weak, a.immune, a.attack_type)

    p_armies = pickle.dumps(armies)

    while True:
        fight(armies)
        alive = set(a.team for a in armies if a.units > 0)
        if 1 == len(alive): break
    for a in armies: a.show()
    print "Remaining units:", sum(a.units for a in armies)

    print

    if argv.hunt:
        boost = [ 0, 10000 ]
        while boost[1] - boost[0] > 1:
            mid = int((boost[1] + boost[0])/2)
            while True:
                armies = pickle.loads(p_armies)
                rv = try_boost(mid, armies)
                if rv is not None: break
                if mid > boost[0]+1:
                    print("Boost {} failed".format(mid))
                    mid -= 1
                else:
                    print "Boost range:", boost
                    exit()
            if 'immune' == rv:
                boost[1] = mid
            else:
                boost[0] = mid
        print "A boost of {} will allow the immune system to win".format(boost[1])
        for a in armies: a.show()

    for b in range(100):
        armies = pickle.loads(p_armies)
        rv = try_boost(b, armies)
        if 'immune' == rv:
            print "Boost {} will do it!".format(b)
            for a in armies: a.show()
            print "Remaining units:", sum(a.units for a in armies)
            exit()


def getopts():
    parser = argparse.ArgumentParser(description="""Advent of code day 24""")
    parser.add_argument('fname', type=str, nargs='?', default="24.in", help='File name')
    parser.add_argument('--hunt')
    return parser.parse_args()

if __name__ == '__main__':
    MAIN(getopts())
