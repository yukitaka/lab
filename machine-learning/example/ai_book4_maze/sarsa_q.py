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

def get_a(s, Q, epsilon, pi_0):
    if np.random.rand() < epsilon:
        return np.random.choice([0, 1, 2, 3], p=pi_0[s])
    else:
        return np.nanargmax(Q[s])

def sarsa(s, a, r, s_next, a_next, Q):
    eta = 0.1
    gamma = 0.9

    if s_next == 8:
        Q[s, a] = Q[s, a] + eta * (r - Q[s, a])
    else:
        Q[s, a] = Q[s, a] + eta * (r + gamma * Q[s_next, a_next] - Q[s, a])
    return Q

