import unittest
from btree import Btree

class TestBtree(unittest.TestCase):
    def test_traverse(self):
            raw_btree = {
                'name': 'a',
                'l': {
                'name': 'b',
                    'l': {'name': 'c'},
                    'r': {'name': 'd', 'l': {'name': 'e'}}
                },
                'r': {'name': 'f'}
            }
            names = []

            def collect_names(x):
                names.append(x.name)
            btree = Btree.create(raw_btree)
            Btree.traverse(btree, collect_names)
            self.assertEquals(['a', 'b', 'c', 'd', 'e', 'f'], names)
