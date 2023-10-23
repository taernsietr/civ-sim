import sys
import matplotlib.pyplot as plt

with open(sys.argv[1], 'r') as f:
    lines = f.readlines()

data = [int(x.strip()) for x in lines]

min_val = min(data)
max_val = max(data)

data = [x for x in data if x >= min_val and x <= max_val]

plt.hist(data, bins=max_val-min_val+1)
plt.savefig(f'{sys.argv[1]}.png')
