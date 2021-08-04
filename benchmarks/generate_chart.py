from matplotlib import pyplot
import numpy as np

values_python = [0.00095796, 0.01006150, 0.1456694, 2.51394, 23.467905]

values_python = [v * 1000 for v in values_python]

values_boson = [0.0014064311, 0.010024547, 0.1464130, 2.2045967, 21.82820]

values_boson = [v * 1000 for v in values_boson]

#X = [values_python, values_boson]

n_calls = [100, 1000, 10000, 100000, 1000000]

ind = np.arange(5) 
width = 0.35   

pyplot.bar(ind, values_python, label="Python")
pyplot.bar(ind + width, values_boson, label="Boson")

pyplot.xlabel("Number of function calls")
pyplot.ylabel("Time taken (ms)")
pyplot.xticks(ind + width / 2, ('100', '1000', '10000', '100000', '1000000'))
pyplot.yticks(np.arange(0, 30 * 1000, 1000))
pyplot.legend(loc='best')

pyplot.show()