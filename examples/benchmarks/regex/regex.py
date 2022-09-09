import re
import time

compiled = re.compile("he.?.?o")

st = time.time()
for _ in range(1000000):
    matches = compiled.findall("hello world! hello everyone!")

et = time.time()
print(et - st)