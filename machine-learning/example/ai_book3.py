from tensorflow.keras.datasets import cifar10
from tensorflow.keras.layers import Activation, Dense, Dropout, Conv2D, Flatten, MaxPool2D
from tensorflow.keras.models import Sequential, load_model
from tensorflow.keras.optimizers import Adam
from tensorflow.keras.utils import to_categorical
import numpy as np
import matplotlib.pyplot as plt

(train_images, train_labels), (test_images, test_labels) = cifar10.load_data()

def data_preview(train_images, train_labels, test_images, test_labels):
    print(train_images.shape)
    print(train_labels.shape)
    print(test_images.shape)
    print(test_labels.shape)

    for i in range(10):
        plt.subplot(2, 5, i+1)
        plt.imshow(train_images[i])
    plt.show()

    print(train_labels[0:10])


data_preview(train_images, train_labels, test_images, test_labels)
