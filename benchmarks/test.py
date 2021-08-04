import time


idx = 0

adder = lambda a,b: a + b

st = time.time()

while idx != 1000000:
    x = adder(30, 10)
    print(x)
    idx = idx + 1
et = time.time()

print('Time: ', et - st)