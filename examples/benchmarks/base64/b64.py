import base64
import time

test_buffer = b'hello, world!'

st = time.time()
for _ in range(0, 1000000):
    x = base64.b64encode(test_buffer)
et = time.time()
print(et - st)