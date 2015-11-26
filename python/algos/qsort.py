import sys

def qsort(l):
    if len(l) <= 1:
        return l
    pivot = l.pop()
    pre_pivot = qsort([x for x in l if x < pivot])
    post_pivot = qsort([x for x in l if x >= pivot])
    return pre_pivot + [pivot] + post_pivot

if __name__ == "__main__":
    args = sys.argv[1:]
    print qsort(args)
