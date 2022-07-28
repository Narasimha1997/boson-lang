import random
import time

x = [ 0 ] * 10000


st = time.time()
for i in range(0, 10000):
    x[i] = random.random()

x.sort()

et = time.time()

print(et - st)