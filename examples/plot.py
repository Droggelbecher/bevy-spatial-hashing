
import matplotlib
from matplotlib import pyplot as plt
import csv
import numpy as np
import sys

COL_GRID_SIZE = 0
COL_QUERY_RADIUS = 1
COL_FRAME_DURATION = 2
COL_POINTS = 3
J = 0.0

def read_data(filename):
    a = np.loadtxt(open(filename, "r"), delimiter=",")

    # Convert from [s] to [ms]
    a[:, COL_FRAME_DURATION] *= 1000.
    return a

def plot_radius_scatter(a):
    n_colors = 12
    colors = plt.cm.viridis(np.linspace(0, 1, n_colors))
    plt.rcParams['axes.prop_cycle'] = plt.cycler("color", colors)

    fig, ax = plt.subplots(figsize=(20, 10), dpi=300)
    ax.grid()

    grid_sizes = sorted(np.unique(a[:, COL_GRID_SIZE]))
    for grid_size in grid_sizes:
        mask = a[:, COL_GRID_SIZE] == grid_size
        data = a[mask]

        if grid_size == -1.0:
            label = "ECS"
        else:
            label = f"$h={grid_size}$"

        color = next(ax._get_lines.prop_cycler)['color']

        jitter = np.random.uniform(low=-J, high=J, size=len(data))
        ax.scatter(data[:, COL_QUERY_RADIUS] + jitter, data[:, COL_FRAME_DURATION], s=10.0,
                   label=label, color=color, marker='^' if grid_size == -1.0 else 'o')

        radii = sorted(np.unique(data[:, COL_QUERY_RADIUS]))
        avg_frame_durations = np.array([
            np.mean(data[data[:, COL_QUERY_RADIUS] == r, COL_FRAME_DURATION])
            for r in radii
        ])
        ax.plot(radii, avg_frame_durations, '--' if grid_size == -1.0 else '-', color=color)

    ax.legend()
    ax.set_xlabel("Query Square Radius ($r$)")
    ax.set_ylabel("Frame Time [ms]")
    ax.set_yscale('log')
    ax.get_yaxis().set_major_formatter(matplotlib.ticker.ScalarFormatter())
    # plt.show()
    plt.savefig("radius.png", bbox_inches="tight", pad_inches=0)

def plot_grid_size_scatter(a):
    fig, ax = plt.subplots(figsize=(20, 10), dpi=300)
    ax.grid()

    n_colors = 10
    colors = plt.cm.viridis(np.linspace(0, 1, n_colors))
    plt.rcParams['axes.prop_cycle'] = plt.cycler("color", colors)

    radii = sorted(np.unique(a[:, COL_QUERY_RADIUS]))
    for radius in radii:
        mask = a[:, COL_QUERY_RADIUS] == radius
        mask &= a[:, COL_GRID_SIZE] != -1
        data = a[mask]
        label = f"$r={radius}$"

        color = next(ax._get_lines.prop_cycler)['color']

        jitter = np.random.uniform(low=-J, high=J, size=len(data))
        ax.scatter(data[:, COL_GRID_SIZE] + jitter, data[:, COL_FRAME_DURATION], s=10.0,
                   label=label, color=color)

        grid_sizes = sorted(np.unique(data[:, COL_GRID_SIZE]))
        avg_frame_durations = np.array([
            np.mean(data[data[:, COL_GRID_SIZE] == r, COL_FRAME_DURATION])
            for r in grid_sizes
        ])
        ax.plot(grid_sizes, avg_frame_durations, '-', color=color)


    ax.legend()
    ax.set_xlabel("Grid Size ($h$)")
    ax.set_ylabel("Frame Time [ms]")
    ax.set_yscale('log')
    ax.get_yaxis().set_major_formatter(matplotlib.ticker.ScalarFormatter())
    # plt.show()
    plt.savefig("grid_size.png", bbox_inches="tight", pad_inches=0)

def plot_grid_size_points(a):
    fig, ax = plt.subplots(figsize=(20, 10), dpi=300)
    ax.grid()

    n_colors = 10
    colors = plt.cm.viridis(np.linspace(0, 1, n_colors))
    plt.rcParams['axes.prop_cycle'] = plt.cycler("color", colors)

    minima_x = []
    minima_y = []

    pointss = sorted(np.unique(a[:, COL_POINTS]))
    for points in pointss:
        mask = a[:, COL_POINTS] == points
        mask &= a[:, COL_GRID_SIZE] != -1
        data = a[mask]
        label = f"$n={points}$"

        color = next(ax._get_lines.prop_cycler)['color']

        jitter = np.random.uniform(low=-J, high=J, size=len(data))
        ax.scatter(data[:, COL_GRID_SIZE] + jitter, data[:, COL_FRAME_DURATION], s=10.0,
                   label=label, color=color)

        grid_sizes = sorted(np.unique(data[:, COL_GRID_SIZE]))
        avg_frame_durations = np.array([
            np.mean(data[data[:, COL_GRID_SIZE] == r, COL_FRAME_DURATION])
            for r in grid_sizes
        ])
        ax.plot(grid_sizes, avg_frame_durations, '-', color=color)

        idx = np.argmin(data[:, COL_FRAME_DURATION])
        minima_x.append(data[idx, COL_GRID_SIZE])
        minima_y.append(data[idx, COL_FRAME_DURATION])

    ax.scatter(minima_x, minima_y, c='k', s=50., marker='s', label='Minima')


    ax.legend()
    ax.set_xlabel("Grid Size ($h$)")
    ax.set_ylabel("Frame Time [ms]")
    # ax.set_yscale('log')
    ax.get_yaxis().set_major_formatter(matplotlib.ticker.ScalarFormatter())
    # plt.show()
    plt.savefig("grid_size_points.png", bbox_inches="tight", pad_inches=0)


if __name__ == '__main__':
    data = read_data(sys.argv[1])
    plot_radius_scatter(data)
    plot_grid_size_scatter(data)
    plot_grid_size_points(data)
