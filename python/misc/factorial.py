import sys

def fact(i):
  def fact2(i, acc):
    if i:
      return fact2(i-1, i*acc)
    else:
      return acc

  return fact2(i, 1)

if __name__ == "__main__":
  i = int(sys.argv[1])
  print fact(i)
