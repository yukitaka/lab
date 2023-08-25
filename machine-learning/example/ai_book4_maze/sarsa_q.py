import numpy as np
import matplotlib.pyplot as plt
from matplotlib import animation
from IPython.display import HTML
import maze as mz

fig, circle, theta_0 = mz.prepare_maze()

def get_pi(theta):
    [m, n] = theta.shape
    pi = np.zeros((m, n))
    for i in range(0, m):
        pi[i, :] = theta[i, :] / np.nansum(theta[i, :])
    pi = np.nan_to_num(pi)
    return pi

pi_0 = get_pi(theta_0)
print(pi_0)

[a, b] = theta_0.shape
Q = np.random.rand(a, b) * theta_0
print(Q)
