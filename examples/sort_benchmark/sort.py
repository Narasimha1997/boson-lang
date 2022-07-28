import random
import time

x = [ 0 ] * 1000000


st = time.time()
for i in range(0, 1000000):
    x[i] = random.random()

x.sort()

et = time.time()

print(et - st)