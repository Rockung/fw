# zip returns a tuple
a = [1,2,3]
b = [4,5,6]
for i,j in zip(a,b):
  print(i,j)
  
# lambda defines a simple function
func = lambda x,y: x+y
print(func(10,20))

# map binds a function with its parameters
def func(x,y):
  return x+y
  
print(list(map(func, [10,30], [20,40])))