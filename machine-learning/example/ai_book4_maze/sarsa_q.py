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

def sarsa(s, a, r, s_next, a_next, Q, eta, gamma):
    if s_next == 8:
        Q[s, a] = Q[s, a] + eta * (r - Q[s, a])
    else:
        Q[s, a] = Q[s, a] + eta * (r + gamma * Q[s_next, a_next] - Q[s, a])
    return Q

def q_learning(s, a, r, s_next, a_next, Q, eta, gamma):
    if s_next == 8:
        Q[s, a] = Q[s, a] + eta * (r -Q[s, a])
    else:
        Q[s, a] = Q[s, a] + eta * (r + gamma * np.nanmax(Q[s_next, :]) - Q[s, a])
    return Q

def play(Q, epsilon, eta, gamma, pi):
    s = 0
    a = a_next = get_a(s, Q, epsilon, pi)
    s_a_history = [[0, np.nan]]

    while True:
        a = a_next
        s_next = mz.get_s_next(s, a)

        s_a_history[-1][1] = a
        s_a_history.append([s_next, np.nan])

        if s_next == 8:
            r = 1
            a_next = np.nan
        else:
            r = 0
            a_next = get_a(s_next, Q, epsilon, pi)

        Q = sarsa(s, a, r, s_next, a_next, Q, eta, gamma)

        if s_next == 8:
            break
        else:
            s = s_next

    return [s_a_history, Q]

eta = 0.1
gamma = 0.9
epsilon = 0.5

for episode in range(10):
    epsilon = epsilon / 2

    [s_a_history, Q] = play(Q, epsilon, eta, gamma, pi_0)

    print('episode: {}, step: {}'.format(episode, len(s_a_history)-1))
