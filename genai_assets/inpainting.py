import torch
import numpy as np
from diffusers import StableDiffusion3InpaintPipeline
from PIL import Image

from genai_assets.config import HUGGING_FACE_HUB_TOKEN, SD3_REPO


def create_gradient_image(image_path, direction, original_fraction, transition_fraction):
    image = Image.open(image_path).convert("RGB")
    image = image.resize((1024, 1024))

    img_array = np.array(image).astype(float)
    height, width, _ = img_array.shape

    original_pixels = int(width * original_fraction)
    transition_pixels = int(width * transition_fraction)

    # Create a gradient
    if direction in ['left', 'right']:
        gradient = np.linspace(0, 1, width)
    else:  # 'up' or 'down'
        gradient = np.linspace(0, 1, height)
        gradient = gradient[:, np.newaxis]

    if direction in ['left', 'up']:
        gradient = 1 - gradient

    # Adjust gradient to match original and transition fractions
    gradient[gradient < original_fraction] = 0
    gradient[gradient > original_fraction + transition_fraction] = 1
    gradient = (gradient - original_fraction) / transition_fraction
    gradient = np.clip(gradient, 0, 1)

    # Apply the gradient to the image
    for c in range(3):  # For each color channel
        img_array[:, :, c] = img_array[:, :, c] * (1 - gradient)  # + 255 * gradient

    gradient_image = Image.fromarray(img_array.astype('uint8'))

    # Create a binary mask
    mask = np.zeros((height, width), dtype=np.uint8)
    if direction == 'left':
        mask[:, original_pixels:-200] = 255
    elif direction == 'right':
        mask[:, :width - original_pixels] = 255
    elif direction == 'up':
        mask[original_pixels:, :] = 255
    elif direction == 'down':
        mask[:height - original_pixels, :] = 255

    mask_image = Image.fromarray(mask)

    return gradient_image, mask_image


def inpaint_edge_texture(image_path, prompt, direction, original_fraction, transition_fraction, seed=None):
    pipeline = StableDiffusion3InpaintPipeline.from_pretrained(
        SD3_REPO,
        text_encoder_3=None,
        tokenizer_3=None,
        torch_dtype=torch.float16,
        token=HUGGING_FACE_HUB_TOKEN,
    ).to("cuda")

    pipeline.enable_model_cpu_offload()

    init_image, mask_image = create_gradient_image(image_path, direction, original_fraction, transition_fraction)

    if seed is not None:
        generator = torch.Generator("cuda").manual_seed(seed)
    else:
        generator = torch.Generator("cuda").manual_seed(torch.randint(0, 1000000, (1,)).item())

    mask_image.save("edge_texture_mask.png")
    init_image.save("edge_texture_init.png")

    image = pipeline(
        prompt=prompt,
        image=init_image,
        mask_image=mask_image,
        generator=generator,
        guidance_scale=7.5,
        num_inference_steps=50,
    ).images[0]

    image.save("edge_texture_output.png")

    return image


# Example usage
if __name__ == "__main__":
    init_image_path = "./terrain_tiles_photo/46_summer_sunny_desert.png"
    # prompt = "A sunny desert landscape viewed from above. Realistic, bold exaggerated styles. Gradually disperses to reveal a dark background."
    prompt = "A sunny desert landscape viewed from above. Realistic, bold exaggerated styles. Hits a dark wall edge."
    direction = "left"
    seed = 92

    original_fraction = 0.3  # How much of the original tile will remain
    inpaint_fraction = 0.3  # How much of the tile will be inpainted (rest is white)

    inpainted_image = inpaint_edge_texture(init_image_path, prompt, direction, original_fraction, inpaint_fraction, seed)
    print("Inpainting complete.")
