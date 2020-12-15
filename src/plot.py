#import matplotlib.pyplot as plt
import seaborn as sns; 
sns.set_theme()
impot 

def generate_heatmap(data):
    ax = sns.heatmap(data, cmap="Spectral")



def load_data(path):
    with open(path, 'r') as file:
        raw_data = json.read()
        json_data = json.loads(raw_data)
        print(json_data)




