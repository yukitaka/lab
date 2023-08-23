import numpy as np
import matplotlib.pyplot as plt
from matplotlib import animation
from IPython.display import HTML

fig = plt.figure(figsize=(3, 3))

plt.plot([0, 3], [3, 3], color='k')
plt.plot([0, 3], [0, 0], color='k')
plt.plot([0, 0], [0, 2], color='k')
plt.plot([3, 3], [1, 3], color='k')
plt.plot([1, 1], [1, 2], color='k')
plt.plot([2, 3], [2, 2], color='k')
plt.plot([2, 1], [1, 1], color='k')
plt.plot([2, 2], [0, 1], color='k')

for i in range(3):
    for j in range(3):
        plt.text(0.5+i, 2.5-j, str(i+j*3), size=20, ha='center', va='center')

circle, = plt.plot([0.5], [2.5], marker='o', color='#d3d3d3', markersize=40)

plt.tick_params(axis='both', which='both', bottom='off', top='off',
        labelbottom='off', right='off', left='off', labelleft='off')
plt.box('off')

theta_0 = np.array([
    [np.nan, 1, 1, np.nan],
    [np.nan, 1, 1, 1],
    [np.nan, np.nan, np.nan, 1],
    [1, np.nan, 1, np.nan],
    [1, 1, np.nan, np.nan],
    [np.nan, np.nan, 1, 1],
    [1, 1, np.nan, np.nan],
    [np.nan, np.nan, np.nan, 1]])

def get_pi(theta):
    [m, n] = theta.shape
    pi = np.zeros((m, n))
    exp_theta = np.exp(theta)
    for i in range(0, m):
        pi[i, :] = exp_theta[i, :] / np.nansum(exp_theta[i, :])
    pi = np.nan_to_num(pi)

    return pi

def get_a(pi, s):
    return np.random.choice([0, 1, 2, 3], p=pi[s])

def get_s_next(s, a):
    if a == 0:
        return s - 3
    elif a == 1:
        return s + 1
    elif a == 2:
        return s + 3
    elif a == 3:
        return s - 1

def play(pi):
    s = 0
    s_a_history = [[0, np.nan]]

    while True:
        a = get_a(pi, s)
        s_next = get_s_next(s, a)
        s_a_history[-1][1] = a
        s_a_history.append([s_next, np.nan])

        if s_next == 8:
            break
        else:
            s = s_next

    return s_a_history

pi_0 = get_pi(theta_0)
s_a_history = play(pi_0)
print(s_a_history)
print('step: {}'.format(len(s_a_history)+1))
