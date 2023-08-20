import numpy as np
import math
import random

class EpsilonGreedy():
    def __init__(self, epsilon):
        self.epsilon = epsilon

    def initialize(self, n_arms):
        self.n = np.zeros(n_arms)
        self.v = np.zeros(n_arms)

    def select_arm(self):
        if self.epsilon > random.random():
            return np.random.randint(0, len(self.v))
        else:
            return np.argmax(self.v)

    def update(self, chosen_arm, reward, t):
        self.n[chosen_arm] += 1
        n = self.n[chosen_arm]
        v = self.v[chosen_arm]
        self.v[chosen_arm] = ((n-1) / float(n)) * v + (1 / float(n)) * reward

    def label(self):
        return 'ε-greedy('+str(self.epsilon)+')'


class UCB1():
    def initialize(self, n_arms):
        self.n = np.zeros(n_arms)
        self.w = np.zeros(n_arms)
        self.v = np.zeros(n_arms)

    def select_arm(self):
        for i in range(len(self.n)):
            if self.n[i] == 0:
                return i

        return np.argmax(self.v)

    def update(self, chosen_arm, reward, t):
        self.n[chosen_arm] += 1
        if reward == 1.0:
            self.w[chosen_arm] += 1

        for i in range(len(self.n)):
            if self.n[i] == 0:
                return

        for i in range(len(self.v)):
            self.v[i] = self.w[i] / self.n[i] + (2 * math.log(t) / self.n[i]) ** 0.5

    def label(self):
        return 'ucb1'

