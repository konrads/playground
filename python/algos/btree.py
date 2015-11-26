# NOTE: would consider making create() and traverse() module methods...

class Btree(object):
    """Binary tree with left and right
    """

    def __init__(self, name, l=None, r=None):
        """initializor

        :param name: node name
        :param l: left side
        :param r: right side
        """
        self.name = name
        self.l = l
        self.r = r

    @staticmethod
    def create(d):
        """creates a Btree wrapper of a dict representation

        :param d: raw dict
        :returns: Btree structure
        """
        if not d:
            return None
        return Btree(d['name'], Btree.create(d.get('l')), Btree.create(d.get('r')))

    @staticmethod
    def traverse(btree, f):
        if not btree:
            return
        f(btree)
        Btree.traverse(btree.l, f)
        Btree.traverse(btree.r, f)


if __name__ == '__main__':
    raw_btree = {
        'name': 'a',
        'l': {
        'name': 'b',
            'l': {'name': 'c'},
            'r': {'name': 'd', 'l': {'name': 'e'}}
        },
        'r': {'name': 'f'}
    }

    def print_names(x):
        print x.name

    btree = Btree.create(raw_btree)
    Btree.traverse(btree, print_names)
