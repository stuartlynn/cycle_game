""" Take a 512x512 image and pixelate it to 64x64, reduce color palette, then upscale again, for use as a retro game asset. """

import argparse
import numpy as np
import os
from PIL import Image, ImageEnhance
from sklearn.cluster import KMeans

INPUT_FOLDER = "generations"
OUTPUT_FOLDER = "retro_" + INPUT_FOLDER


def kmeans_color_quantization(image, num_colors):
    # Reshape the image to be a list of pixels
    pixels = np.array(image).reshape(-1, 3)

    # Perform k-means clustering
    kmeans = KMeans(n_clusters=num_colors, random_state=42)
    labels = kmeans.fit_predict(pixels)

    # Get the colors
    colors = kmeans.cluster_centers_.astype(int)

    # Map each pixel to the closest color
    quantized_pixels = colors[labels]

    # Reshape back to the original image shape
    quantized_image = quantized_pixels.reshape(image.size[1], image.size[0], 3)

    return Image.fromarray(quantized_image.astype('uint8'))


def pixelate_and_reduce_colors(img, target_size=(64, 64), num_colors=16, palette_size=None):

    img = img.convert('RGB')
    original_size = img.size

    # boost saturation of image
    sat_booster = ImageEnhance.Color(img)
    img = sat_booster.enhance(1.25)

    # increase contrast of image
    contr_booster = ImageEnhance.Contrast(img)
    img = contr_booster.enhance(1.25)

    pixelated = img.resize(target_size, resample=Image.NEAREST)

    # Use k-means for initial color reduction
    reduced_color = kmeans_color_quantization(pixelated, num_colors)

    if palette_size is not None and palette_size < num_colors:
        # Use k-means again for further palette reduction
        reduced_color = kmeans_color_quantization(reduced_color, palette_size)

    return reduced_color.resize(original_size, resample=Image.NEAREST)


def process_images(input_folder, output_folder, num_colors=16, palette_size=None):
    if not os.path.exists(output_folder):
        os.makedirs(output_folder)

    for filename in os.listdir(input_folder):
        if filename.lower().endswith(('.png', '.jpg', '.jpeg', '.bmp', '.gif')):
            input_path = os.path.join(input_folder, filename)
            output_path = os.path.join(output_folder, f"retro_{filename}")

            try:
                with Image.open(input_path) as img:
                    processed_img = pixelate_and_reduce_colors(img, num_colors=num_colors, palette_size=palette_size)
                    processed_img.save(output_path)
                print(f"Processed: {filename}")
            except Exception as e:
                print(f"Error processing {filename}: {str(e)}")


def main():
    parser = argparse.ArgumentParser(description="Batch generate retro game assets from modern images.")
    parser.add_argument("--input_folder", default=INPUT_FOLDER,
                        help="Path to the folder containing input images (default: input_images)")
    parser.add_argument("--output_folder", default=OUTPUT_FOLDER,
                        help="Path to the folder to save output images (default: output_images)")
    parser.add_argument("--colors", type=int, default=16,
                        help="Number of colors in the initial reduced palette (default: 16)")
    parser.add_argument("--palette", type=int, help="Further reduce palette to this number of colors (optional)")

    args = parser.parse_args()

    process_images(args.input_folder, args.output_folder, num_colors=args.colors, palette_size=args.palette)
    print(f"Retro game assets saved to {args.output_folder}")


if __name__ == "__main__":
    main()
