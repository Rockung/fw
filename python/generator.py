def fib(max):
  a,b = 0,1
  while max:
    r = b
    a,b = b, a+b
    max -= 1
    yield r

# python generator.py
if __name__ == '__main__':
  for i in fib(5):
    print(i)
