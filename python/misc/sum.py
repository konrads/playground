import sys

def sum(f, a, b):
  def sum2(a, acc):
    if a>b:
      return acc
    else:
      return sum2(a+1, f(a)+acc)
  
  return sum2(a, 0)

def square(a):
  """Same as lambda x: x*x"""
  return a*a

if __name__ == "__main__":
  start = int(sys.argv[1])
  end = int(sys.argv[2])
  # print sum(square, start, end)
  print sum(lambda x: x*x, start, end)
