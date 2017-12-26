def fib(max):
  a,b = 0,1
  while max:
    r = b
    a,b = b, a+b
    max -= 1
    yield r
    
for i in fib(5):
  print(i)
