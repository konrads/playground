import unittest
from fib import fib

class TestFibo(unittest.TestCase):
    def test_fib(self):
        self.assertEquals(0, fib(0))
        self.assertEquals(1, fib(1))
        self.assertEquals(1, fib(2))
        self.assertEquals(2, fib(3))
        self.assertEquals(3, fib(4))
        self.assertEquals(5, fib(5))
        self.assertEquals(8, fib(6))
        self.assertEquals(13, fib(7))
