# l = [1,4,7,10]
# def a(acc,x):
#     if not x % 2:
#         return acc
#     else:
#         return acc+x
# print reduce(a, l)  # prints 1+7=8

tree = ('a', [
  ('b', [('c', []), ('d', [])]),
  ('e', [('f', []), ('g', [])]),
])

def depth_first((node, children)):
  yield node
  for c in children:
    for cc in depth_first(c):
      yield cc


def breadth_first((node, children)):
  for (c_node,_) in children:
    yield c_node
  for c in children:
    for cc in breadth_first(c):
      yield cc


if __name__ == "__main__":
  print "depth_first:", list(depth_first(tree))
  print
  print "breath_first:", list(breadth_first(tree))
