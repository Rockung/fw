import numpy as np

a = np.arange(12).reshape((3,4))

b = np.split(a,2,axis=1) # vertical splitting
c = np.split(a,3,axis=0) # horizontal splitting

np.vsplit(a,3)
np.hsplit(a,2)

np.array_split(a,3,axis=1)
