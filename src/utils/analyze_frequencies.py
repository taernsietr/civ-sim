import sys
import matplotlib.pyplot as plt

with open(sys.argv[1], 'r') as f:
    lines = f.readlines()[2:]

def scale(x):
  return int(((float(x)+1)/2) * 255)

data = [x.strip().split(",")[1:] for x in lines]
joined = [scale(x) for y in data for x in y]

plt.hist(joined, bins=255)
