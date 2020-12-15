import matplotlib.pyplot as plt
import seaborn as sns; 
import json
import numpy as np
import pandas as pd

def generate_heatmap(data, iteration, arr_min, arr_max):
    plt.cla()
    plt.clf()

    ax = sns.heatmap(data, cmap="coolwarm", vmin=arr_min, vmax=arr_max)
    ax.tick_params(bottom=False)
    ax.tick_params(left=False)

    # pad the number with zeros for ffmpeg
    padded = str(iteration).zfill(3)

    plt.savefig(f"figures/{padded}.png")


def error_plot(error_information, error_name, collection_rate):
    plt.cla()
    plt.clf()
    plot = sns.lineplot(data= error_information)

    plt.yscale('log')
    plt.xlabel("Step")
    plt.ylabel(error_name)
    plt.title("Error over time")

    ticks = plot.get_xticks()
    xlabels = [int(collection_rate * i) for i in ticks]

    plot.set_xticklabels(xlabels)

    for item in plot.get_xticklabels():
        item.set_rotation(0)

    plt.savefig(f"figures/error.png")

def load_data(path):
    with open(path, 'r') as file:
        raw_data = file.read()
        json_data = json.loads(raw_data)
        return json_data

if __name__ == "__main__":
    all_data = load_data("results.json")
    data =all_data["simulation"]

    arr = np.asarray(data["step_data"][-1]["data"])
    size = data["size"]
    arr = arr.reshape((size, size, size))
    arr_min = arr.min()
    arr_max = arr.max() 
    print(f"min {arr_min} max {arr_max}")

    for i in range(0, size):
        left = arr[i,:,:]
        generate_heatmap(left, i, arr_min, arr_max)


    # plot the error over time
    error_info = pd.Series(data["error_decay"]["data"])
    error_name = data["error_decay"]["error_type"]
    error_collection_rate = all_data["solver_params"]["error_steps"]
    print(error_collection_rate)

    error_plot(error_info, f"{error_name} value", error_collection_rate)
