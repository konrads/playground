"""
Simulate the "Monty Hall" Problem.

In this problem, the contestant is asked to choose one of three doors. Behind one of the doors is a new car. 
Behind each of the others is a goat. After the contestant picks a door the host, who knows what is behind all the doors, 
opens one of the other doors to reveal a goat and gives the contestant a chance to stick with their original selection, 
or switch to the remaining unopened door. It has been proven that switching doors gives the contestant a 33% better chance
of winning the car than sticking with the first choice. 

Assignment: Write a program that simulates this game (including the host/contestant choices). The program takes as 
input an integer value representing the number of trials and returns a double representing the percentage of times
that the switching strategy would successfully result in winning the car.
"""

from random import randrange


def run_game(should_switch, choice):
    doors = [False] * 3
    doors[randrange(3)] = True
    if not should_switch:
        return doors[choice]
    else:
        del doors[choice]  # not choosing initial door
        host_choice = next(for d in doors if not d)
        del doors[host_choice]
        return doors[0]  # remaining door


def run_many(should_switch, count):
    wins = sum(run_game(should_switch, randrange(3)) for x in xrange(count))
    win_ratio = wins/float(count)
    print 'Switching strategy: %r, result: %f' % (should_switch, win_ratio)


run_many(True, 1000)
