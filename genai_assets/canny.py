""" Creates control outline from an image. """

from PIL import Image

import numpy as np
import cv2


def make_control_image(ref_image, low_threshold=100, high_threshold=200):
    """ Creates control outline from an image. """

    image = np.array(ref_image)

    image = cv2.Canny(image, low_threshold, high_threshold)
    image = image[:, :, None]
    image = np.concatenate([image, image, image], axis=2)
    return Image.fromarray(image)

