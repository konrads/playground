import unittest
from qsort import qsort


class TestQSort(unittest.TestCase):
    def test_empty(self):
        self.assertEquals([], qsort([]))

    def test_1_elem(self):
        self.assertEquals([5], qsort([5]))

    def test_2_elems(self):
        self.assertEquals([3, 5], qsort([5, 3]))

    def test_many_elems(self):
        self.assertEquals([1, 2, 3, 3, 5, 9], qsort([5, 3, 9, 3, 2, 1]))
