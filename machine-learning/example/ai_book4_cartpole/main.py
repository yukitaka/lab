import gym
import numpy as np
from keras.models import Sequential
from keras.layers import Dense
from keras.optimizers import Adam
from collections import deque
from tensorflow.keras.losses import Huber

NUM_EPISODES = 500
MAX_STEPS = 200
GAMMA = 0.99
WARMUP = 10

E_START = 1.0
E_STOP = 0.01
E_DECAY_RATE = 0.001

MEMORY_SIZE = 10000
BATCH_SIZE = 32

class QNetwork:
    def __init__(self, state_size, action_size):
        self.model = Sequential()
        self.model.add(Dense(16, activation='relu', input_dim=state_size))
        self.model.add(Dense(16, activation='relu'))
        self.model.add(Dense(16, activation='relu'))
        self.model.add(Dense(action_size, activation='linear'))

        self.model.compile(loss=Huber(delta=100.0), optimizer=Adam(learning_rate=0.001))

env = gym.make('CartPole-v0')
state_size = env.observation_space.shape[0]
action_size = env.action_space.n

main_qn = QNetwork(state_size, action_size)
target_qn = QNetwork(state_size, action_size)
