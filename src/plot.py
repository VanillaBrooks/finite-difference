import matplotlib.pyplot as plt
import seaborn as sns; 
import json
import numpy as np

def generate_heatmap(data, iteration, arr_min, arr_max):
    plt.cla()
    plt.clf()

    ax = sns.heatmap(data, cmap="coolwarm", vmin=arr_min, vmax=arr_max)
    plt.savefig(f"figures/{iteration}.png")


def load_data(path):
    with open(path, 'r') as file:
        raw_data = file.read()
        json_data = json.loads(raw_data)
        return json_data

if __name__ == "__main__":
    data = load_data("results.json")
    arr = np.asarray(data["step_data"][-1]["data"])
    size = data["size"]
    arr = arr.reshape((size, size, size))
    arr_min = arr.min()
    arr_max = arr.max() *.9999
    print(f"min {arr_min} max {arr_max}")

    for i in range(0, size):
        left = arr[i,:,:]
        generate_heatmap(left, i, arr_min, arr_max)

