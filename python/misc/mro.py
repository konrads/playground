class A(object):
    def __init__(self, x):
        print 'A.x', x

class B(A):
    def __init__(self, x):
        print 'B.x', x

class C(A):
    def __init__(self, x):
        print 'C.x', x

class D(B, C):
    def __init__(self, x):
        print 'D.x', x
        super(D, self).__init__(x)

d = D(10)  # prints D.x 10, B.x 10!  No C.x or A.x...
