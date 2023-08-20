import numpy as np
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

