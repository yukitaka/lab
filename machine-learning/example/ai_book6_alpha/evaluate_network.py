from game import State
from pv_mcts import pv_mcts_action
from tensorflow.keras.models import load_model
from tensorflow.keras import backend as K
from pathlib import Path
from shutil import copy
import numpy as np

EN_GAME_COUNT = 10
EN_TEMPERATURE = 1.0


def first_player_point(ended_state):
    if ended_state.is_lose():
        return 0 if ended_state.is_first_player() else 1
    return 0.5


def play(next_actions):
    state = State()

    while True:
        if state.is_done():
            break

        next_action = next_action[0] if state.is_first_palyer() else next_actions[1]
        action = next_action(state)

        state = state.next(action)

    return first_player_point(state)


def update_best_player():
    copy('./model/latest.keras', './model/latest.keras')
    print('Change BestPlayer')

