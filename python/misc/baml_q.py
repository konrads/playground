'''
Id:          "$Id: dave_test.py,v 1.1 2013/02/05 10:52:23 salim.fadhley Exp $"
Description:
Test:
'''

"""
Given a list of probabilities (floats), and a list of values (anything), write a function/class/generator/whatever:

go(probs, values)

that produces the values with the given probabilities.
For example, you'd expect calling:

go([0.3, 0.7], ['hello', 'world'])

It's a random object chooser: Invoked 100 times would give (roughly, but probably not exactly) 30 'hello's and 70 'world's, in some order.

Consider that you don't know in advance how many times 'go' is going to be called.

There are many ways to write this, many interfaces -- go could be a function, a class, whatever you want.

How we'd like you to solve the problem:
    * Simple stuff that works is better than ambitious but incomplete
    * We want to see test-driven development (you can use unittest2, nose, mock)
    * Try to use 'pythonic' idioms. Consider what might look best to an experienced Python developer

"""
#import unittest2
import unittest
import random
#from qz.core.test.qzunittest import qzUnitTest

def go(probs, values, rand_num=None):
    if not rand_num:
        rand_num = random.random()
    acc_prob = 0
    for prob, val in zip(probs, values):
        acc_prob += prob
        if rand_num <= acc_prob:
            return val
    assert acc_prob <= 1

#class Test(unittest2.TestCase):
class Test(unittest.TestCase):
    def test_signature(self):
        go([0.3, 0.7], ['hello', 'world'])

    def test_response_set(self):
        val = go([0.3, 0.7], ['hello', 'world'])
        self.assertIn(val, ['hello', 'world'])

    def test_100_perc(self):
        val = go([0, 1], ['hello', 'world'])
        self.assertEquals('world', val)
        val = go([0, 1], ['hello', 'world'], 0)
        self.assertEquals('world', val)
        val = go([0, 1], ['hello', 'world'], 0.99999999)  # random cannot hit 1.0
        self.assertEquals('world', val)

    def test_0_perc(self):
        val = go([1, 0], ['hello', 'world'])
        self.assertEquals('hello', val)
        val = go([1, 0], ['hello', 'world'], 0)
        self.assertEquals('hello', val)
        val = go([1, 0], ['hello', 'world'], 0.99999999)  # random cannot hit 1.0
        self.assertEquals('hello', val)

    def test_known_random(self):
        val = go([0.5, 0.5], ['hello', 'world'], 0.1)
        self.assertEquals('hello', val)
        val = go([0.5, 0.5], ['hello', 'world'], 0.8)
        self.assertEquals('world', val)

    def test_multiple_lengths(self):
        val = go([0.2, 0.2, 0.1, 0.5], ['hello', 'wonderful', 'world', '!'], 0.45)
        self.assertEquals('world', val)
        val = go([0.1, 0.7, 0.2], ['hello', 'world', '!'], 0.9)
        self.assertEquals('!', val)
        val = go([1], ['hello'], 0.3)
        self.assertEquals('hello', val)

    def test_probs_dont_sum_up(self):
        self.assertRaises(AssertionError, go, [0.2, 0.2, 0.1, 999], ['hello', 'wonderful', 'world', '!'], 9999999)



def main():
    pass
    # qzUnitTest(headless=True, suppressOutput=False)
