from tensorflow.keras.datasets import cifar10
from tensorflow.keras.models import Model
from tensorflow.keras.utils import to_categorical
from tensorflow.keras.preprocessing.image import ImageDataGenerator
from tensorflow import keras
import numpy as np
import matplotlib.pyplot as plt

model = keras.models.load_model('resnet.keras')

(t1,t2), (test_images, test_labels) = cifar10.load_data()
test_labels = to_categorical(test_labels)

test_gen = ImageDataGenerator(
        featurewise_center=True,
        featurewise_std_normalization=True)
test_gen.fit(test_images)

batch_size = 128
test_loss, test_acc = model.evaluate(
        test_gen.flow(test_images, test_labels, batch_size=batch_size),
        steps=10)

print('loss: {:.3f}\nacc: {:.3f}'.format(test_loss, test_acc))

for i in range(10):
    plt.subplot(2, 5, i+1)
    plt.imshow(test_images[i])
plt.show()

test_predictions = model.predict(
        test_gen.flow(test_images[0:10], shuffle = False, batch_size=1), steps=10)
test_predictions = np.argmax(test_predictions, axis=1)
labels = ['airplane', 'automobile', 'bird', 'cat', 'deer', 'dog', 'frog', 'horse', 'ship', 'truck']
print([labels[n] for n in test_predictions])
