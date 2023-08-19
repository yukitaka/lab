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


def residual_block(filters, strides, unit_size):
    def f(x):
        x = first_residual_unit(filters, strides)(x)
        for i in range(unit_size-1):
            x = residual_unit(filters)(x)
        return x
    return f


input = Input(shape=(32, 32, 3))
x = conv(16, 3)(input)

x = residual_block(64, 1, 18)(x)
x = residual_block(128, 2, 18)(x)
x = residual_block(256, 2, 18)(x)

x = BatchNormalization()(x)
x = Activation('relu')(x)

x = GlobalAveragePooling2D()(x)

output = Dense(10, activation='softmax', kernel_regularizer=l2(0.0001))(x)

model = Model(inputs=input, outputs=output)
model.compile(loss='categorical_crossentropy', optimizer=SGD(momentum=0.9), metrics=['acc'])

train_gen = ImageDataGenerator(
        featurewise_center=True,
        featurewise_std_normalization=True,
        width_shift_range=0.125,
        height_shift_range=0.125,
        horizontal_flip=True)
test_gen = ImageDataGenerator(
        featurewise_center=True,
        featurewise_std_normalization=True)

for data in (train_gen, test_gen):
    data.fit(train_images)


def step_decay(epoch):
    x = 0.1
    if epoch >= 80: x = 0.01
    if epoch >= 120: x = 0.001
    return x


lr_decay = LearningRateScheduler(step_decay)

batch_size = 128
history = model.fit(
        train_gen.flow(train_images, train_labels, batch_size=batch_size),
        epochs=200,
        steps_per_epoch=train_images.shape[0] // batch_size,
        validation_data=test_gen.flow(test_images, test_labels, batch_size=batch_size),
        validation_steps=test_images.shape[0] // batch_size,
        callbacks=[lr_decay])

model.save('resnet.keras')

plt.plot(history.history['acc'], label='acc')
plt.plot(history.history['val_acc'], label='val_acc')
plt.ylabel('accuracy')
plt.xlabel('epoch')
plt.legend(loc='best')
plt.show()
