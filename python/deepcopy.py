import copy

a = [1,2,[3,4]] # the third element[2] is an internal one

# assignment: have the same ID
b = a
print(id(a)==id(b))
b[0] = 100
print(a,b)

# shadow copy: have the same internal elements
c = copy.copy(a)
print(id(c)==id(a))
print(id(c[2])==id(a[2]))
c[0] = 200
print(a,c)

# deep copy: nothing is the same
d = copy.deepcopy(a)
print(id(d)==id(a))
print(id(d[2])==id(a[2]))
d[0] = 300
print(a,d)
