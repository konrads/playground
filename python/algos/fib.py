import sys

def fib(i):
    def fib2(i1, i2, cnt):
        if cnt:
            return fib2(i2, i1+i2, cnt-1)
        return i2
    return fib2(0, 1, i)

if __name__ == "__main__":
    i = int(sys.argv[1])
    print fib(i)
