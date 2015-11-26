class A(object):
    _name = 'a'  # inherited by self

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, name):
        self._name = name


a = A()
print 'a.name: ' + a.name
a.name = 'b'
print 'new a.name: ' + a.name
