import numpy as np

a = np.array([1,1,1])[:,np.newaxis]
b = np.array([2,2,2])[:,np.newaxis]

c = np.concatenate((a,b,b,a),axis=0)
d = np.concatenate((a,b,b,a),axis=1)
print(c)
print(d)
