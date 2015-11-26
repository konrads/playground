def fizzbuzz(x):
    res = 'Fizz' if not x % 3 else ''
    if not x % 5:
        res += 'Buzz'
    if not res:
        res = x
    return res

for x in xrange(1, 101):
    print fizzbuzz(x)