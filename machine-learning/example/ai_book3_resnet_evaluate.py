from tensorflow.keras.datasets import cifar10
from tensorflow.keras.models import Model
from tensorflow.keras.utils import to_categorical
from tensorflow.keras.preprocessing.image import ImageDataGenerator
from tensorflow import keras

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
