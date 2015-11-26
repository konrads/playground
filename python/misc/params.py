def do_stuff(x, y=99, *args, **kwargs):
    print 'x: %s, y: %s, args: %r, kwargs: %r' % (x, y, args, kwargs)


if __name__ == '__main__':
    do_stuff(1, 2, 3)
    do_stuff(1, 2, zzz=999)
    do_stuff(x=1, y=2, z=3)
    # fails -- do_stuff(x=1, y=2, 9)
    do_stuff(**{'x':'hello', 'first':'Dan'})