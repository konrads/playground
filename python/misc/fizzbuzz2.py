import sys
import itertools

def fizzbuzz(i):
  fizzes = itertools.cycle(["", "", "fizz"])
  buzzes = itertools.cycle(["", "", "", "", "buzz"])
  for _, f, b in zip(xrange(i), fizzes, buzzes):
    yield f + b

if __name__ == "__main__":
  i = int(sys.argv[1])
  print list(fizzbuzz(i))
