from matplotlib import pyplot as plt
import numpy as np

fig, ax = plt.subplots()

xs = np.random.uniform(low=-950, high=950, size=10000)
ys = np.random.uniform(low=-500, high=500, size=10000)

ax.grid()
ax.set_aspect("equal", adjustable="datalim")

ax.scatter(xs, ys, s=1.0, c='k')
plt.show()
