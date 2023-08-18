from tensorflow.keras.datasets import cifar10
from tensorflow.keras.callbacks import LearningRateScheduler
from tensorflow.keras.layers import Activation, Add, BatchNormalization, Conv2D, Dense, GlobalAveragePooling2D, Input, Dropout
from tensorflow.keras.models import Model
from tensorflow.keras.optimizers import SGD
from tensorflow.keras.preprocessing.image import ImageDataGenerator
from tensorflow.keras.regularizers import l2
from tensorflow.keras.utils import to_categorical
import numpy as np
import matplotlib.pyplot as plt

(train_images, train_labels), (test_images, test_labels) = cifar10.load_data()

train_labels = to_categorical(train_labels)
test_labels = to_categorical(test_labels)

input = Input(shape=(784,))
x = Dense(256, activation='sigmoid')(input)
x = Dense(128, activation='sigmoid')(x)
x = Dropout(rate=0.5)(x)
x = Dense(10, activation='softmax')(x)
model = Model(inputs=input, outputs=x)

def conv(filters, kernel_size, strides=1):
    return Conv2D(filters, kernel_size, strides=strides, padding='same', use_bias=False, kernel_initializer='he_normal', kernel_regularizer=l2(0.0001))


def first_residual_unit(filters, strides):
    def f(x):
        # ->BN->ReLU
        x = BatchNormalization()(x)
        b = Activation('relu')(x)

        # Convolutional layer->BN->ReLU
        x = conv(filters // 4, 1, strides)(b)
        x = BatchNormalization()(x)
        x = Activation('relu')(x)

        # Convolutional layer->BN->ReLU
        x = conv(filters // 4, 3)(x)
        x = BatchNormalization()(x)
        x = Activation('relu')(x)

        # Convolutional layer->
        x = conv(filters, 1)(x)

        sc = conv(filters, 1, strides)(b)

        return Add()([x, sc])
    return f


def residual_unit(filters):
    def f(x):
        sc = x
        # ->BN->ReLU
        x = BatchNormalization()(x)
        x = Activation('relu')(x)

        # Convolutional layer->BN->ReLU
        x = conv(filters // 4, 1)(x)
        x = BatchNormalization()(x)
        x = Activation('relu')(x)

        # Convolutional layer->BN->ReLU
        x = conv(filters // 4, 3)(x)
        x = BatchNormalization()(x)
        x = Activation('relu')(x)

        # Convolutional layer->
        x = conv(filters, 1)(x)

        return Add()([x, sc])
    return f

