import time

def fib(N):

    if N == 0:
        return 0
    if N == 1:
        return 1
    
    return fib(N - 1) + fib(N - 2)


st = time.time()
result = fib(50)
et = time.time()

print(result, et - st)