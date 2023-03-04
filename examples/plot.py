
from matplotlib import pyplot as plt
import csv
import numpy as np
import sys

COL_GRID_SIZE = 0
COL_QUERY_RADIUS = 1
COL_FRAME_DURATION = 2

def read_data(filename):
    a = np.loadtxt(open(filename, "r"), delimiter=",")

    # Convert from [s] to [ms]
    a[:, COL_FRAME_DURATION] *= 1000.
    return a

def plot_radius_scatter(a):

    fig, ax = plt.subplots(figsize=(20, 10), dpi=300)
    ax.grid()

    grid_sizes = sorted(np.unique(a[:, COL_GRID_SIZE]))
    print(grid_sizes)

    for grid_size in grid_sizes:
        mask = a[:, COL_GRID_SIZE] == grid_size
        data = a[mask]

        if grid_size == -1.0:
            label = "ECS"
        else:
            label = f"Grid Size h={grid_size}"

        color = next(ax._get_lines.prop_cycler)['color']

        J = 0.5
        jitter = np.random.uniform(low=-J, high=J, size=len(data))
        ax.scatter(data[:, COL_QUERY_RADIUS] + jitter, data[:, COL_FRAME_DURATION], s=10.0,
                   label=label, color=color)

        radii = sorted(np.unique(data[:, COL_QUERY_RADIUS]))
        avg_frame_durations = np.array([
            np.mean(data[data[:, COL_QUERY_RADIUS] == r, COL_FRAME_DURATION])
            for r in radii
        ])
        ax.plot(radii, avg_frame_durations, '--', color=color)


    ax.legend()
    ax.set_xlabel("Query Square Radius (r)")
    ax.set_ylabel("Frame Time [ms]")
    ax.set_yscale('log')
    # plt.show()
    plt.savefig("plot.png")

if __name__ == '__main__':
    data = read_data(sys.argv[1])
    plot_radius_scatter(data)
