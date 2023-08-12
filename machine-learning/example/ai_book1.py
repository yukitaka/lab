from tensorflow.keras.datasets import boston_housing
from tensorflow.keras.layers import Activation, Dense, Dropout
from tensorflow.keras.models import Sequential
from tensorflow.keras.callbacks import EarlyStopping
from tensorflow.keras.optimizers import Adam
import pandas as pd
import numpy as np
import matplotlib.pyplot as plot

(train_data, train_labels), (test_data, test_labels) = boston_housing.load_data()

def count_shapes(train_data, train_labels, test_data):
    print("Count shapes")
    print(train_data.shape)
    print(train_labels.shape)
    print(test_data.shape)


def view_tables(train_data):
    print("View tables")
    column_names = ['CRIM', 'ZN', 'INDUS', 'CHAS', 'NOX', 'RM', 'AGE', 'DIS', 'RAD', 'TAX', 'PTRATIO', 'B', 'LSTAT']
    df = pd.DataFrame(train_data, columns=column_names)
    df.head()

    print(df)


def confirm_labels(train_labels):
    print("Confirm labels")
    print(train_labels[0:10])


count_shapes(train_data, train_labels, test_data)
view_tables(train_data)
confirm_labels(train_labels)

