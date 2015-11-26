import sys

def is_anagram(s):
  if len(s) <= 1:
    return True
  elif s[0] != s[-1]:
    return False
  else:
    return is_anagram(s[1:-1])

if __name__ == "__main__":
  s = sys.argv[1]
  print is_anagram(s)
