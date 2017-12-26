import numpy as np

# a = np.array([1,1,1])
# b = np.array([2,2,2])

a = np.array([1,1,1])[:,np.newaxis]
b = np.array([2,2,2])[:,np.newaxis]

c = np.vstack((a,b))
d = np.hstack((a,b))
print(d)
print(c.shape,d.shape)
