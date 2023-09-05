from dual_network import DN_INPUT_SHAPE
from tensorflow.keras.callbacks import LearningRateScheduler, LambdaCallback
from tensorflow.keras.models import load_model
from tensorflow.keras import backend as K
from pathlib import Path
import numpy as np
import pickle

RN_EPOCHS = 100


def load_data():
  history_path = sorted(Path('./data').glob('*.history'))[-1]
  with history_path.open(mode='rb') as f:
    return pickle.load(f)
