class Old():
    pass

class New(object):
    pass


old = Old()
new = New()
print 'old type: %s, class: %s' % (type(old), old.__class__)  # differ: <type 'instance'>, class: __main__.Old
print 'new type: %s, class: %s' % (type(new), new.__class__)  # same: <class '__main__.New'>, class: <class '__main__.New'>