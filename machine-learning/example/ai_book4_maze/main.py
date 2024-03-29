import numpy as np
import matplotlib.pyplot as plt
from matplotlib import animation
from IPython.display import HTML
import maze as mz

fig, circle, theta_0 = mz.prepare_maze()

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

def play(pi):
    s = 0
    s_a_history = [[0, np.nan]]

    while True:
        a = get_a(pi, s)
        s_next = mz.get_s_next(s, a)
        s_a_history[-1][1] = a
        s_a_history.append([s_next, np.nan])

        if s_next == 8:
            break
        else:
            s = s_next

    return s_a_history

def update_theta(theta, pi, s_a_history):
    eta = 0.1
    total = len(s_a_history) - 1
    [s_count, a_count] = theta.shape

    delta_theta = theta.copy()
    for i in range(0, s_count):
        for j in range(0, a_count):
            if not(np.isnan(theta[i, j])):
                sa_ij = [sa for sa in s_a_history if sa == [i, j]]
                n_ij = len(sa_ij)

                sa_i = [sa for sa in s_a_history if sa[0] == i]
                n_i = len(sa_i)

                delta_theta[i, j] = (n_ij - pi[i, j] * n_i) / total

    return theta + eta * delta_theta


stop_epsilon = 10**-4
theta = theta_0
pi = get_pi(theta_0)
s_a_history = play(pi)

for episode in range(10000):
    s_a_history = play(pi)
    theta = update_theta(theta, pi, s_a_history)
    pi_new = get_pi(theta)
    pi_delta = np.sum(np.abs(pi_new-pi))
    pi = pi_new

#    print('episode: {}, step: {}, policy change: {:.4f}'.format(
#        episode, len(s_a_history)-1, pi_delta))

    if pi_delta < stop_epsilon:
        break


def animate(i):
    state = s_a_history[i][0]
    circle.set_data((state % 3) + 0.5, 2.5 - int(state / 3))
    return circle

anim = animation.FuncAnimation(fig, animate, \
        frames=len(s_a_history), interval=200, repeat=False)
HTML(anim.to_jshtml())

