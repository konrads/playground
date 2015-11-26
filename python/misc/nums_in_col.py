#anagram detection
#factorial
#print nums in column
import sys

def in_col(args):
  m = max(args)
  m_len = len(str(m))
  as_str = [str(x) for x in args]
  as_col = [indent(m_len, x)+x for x in as_str]
  return as_col

def indent(m_len, x):
  return " " * (m_len-len(x))

if __name__ == "__main__":
  args = [int(x) for x in sys.argv[1:]]
  print in_col(args)
