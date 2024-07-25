import cv2
import os
from PIL import Image

from diffusion.pixelate import pixelate_and_reduce_colors

OUTPUT_FOLDER = "retro_frames"
SOURCE_VIDEO = "C:/Users/fergu/Downloads/stu.mp4"


def pixelate_video(input_video, output_folder=OUTPUT_FOLDER, num_colors=16, palette_size=None, scaling_factor: int = 8):

    if not os.path.exists(output_folder):
        os.makedirs(output_folder)

    cap = cv2.VideoCapture(input_video)
    frame_count = 0
    w, h = cap.get(cv2.CAP_PROP_FRAME_WIDTH), cap.get(cv2.CAP_PROP_FRAME_HEIGHT)
    target_resolution = int(w // scaling_factor), int(h // scaling_factor)

    while True:
        ret, frame = cap.read()
        if not ret:
            break

        # Convert BGR to RGB
        frame_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        pil_image = Image.fromarray(frame_rgb)

        processed_img = pixelate_and_reduce_colors(pil_image, target_size=target_resolution, num_colors=num_colors, palette_size=palette_size)

        output_path = os.path.join(output_folder, f"frame_{frame_count:04d}.png")
        processed_img.save(output_path)

        print(f"Processed: frame_{frame_count:04d}.png")
        frame_count += 1

    cap.release()
    print(f"Processed {frame_count} frames. Retro frames saved to {output_folder}")


def main(input_video, output_folder=OUTPUT_FOLDER, num_colors=16, palette_size=None):
    pixelate_video(input_video, output_folder, num_colors, palette_size)


if __name__ == "__main__":
    # Example usage
    main(SOURCE_VIDEO, OUTPUT_FOLDER, num_colors=16, palette_size=10)
