import torch
from diffusers import AutoPipelineForInpainting, StableDiffusion3InpaintPipeline
from PIL import Image, ImageDraw

from genai_assets.config import HUGGING_FACE_HUB_TOKEN, SD3_REPO


def create_edge_texture(image_path, direction, original_fraction, inpaint_fraction):

    # Load and resize the image
    image = Image.open(image_path).convert("RGB")
    # image = image.resize((512, 512))

    # Calculate pixels for each region
    original_pixels = int(512 * original_fraction)
    inpaint_pixels = int(512 * inpaint_fraction)
    white_pixels = 512 - original_pixels - inpaint_pixels

    # Create the new image and mask
    new_image = Image.new('RGB', (512, 512), (255, 255, 255))
    mask = Image.new('L', (512, 512), 0)  # 0 is black (area to keep), 255 is white (area to inpaint)

    if direction == 'left':
        extra = int(inpaint_pixels // 2)
        new_image.paste(image.crop((0, 0, original_pixels + extra, 512)), (0, 0))
        ImageDraw.Draw(mask).rectangle([original_pixels, 0, original_pixels + inpaint_pixels, 512], fill=255)
    elif direction == 'right':
        new_image.paste(image.crop((512 - original_pixels, 0, 512, 512)), (512 - original_pixels, 0))
        ImageDraw.Draw(mask).rectangle([white_pixels, 0, 512 - original_pixels, 512], fill=255)
    elif direction == 'up':
        new_image.paste(image.crop((0, 0, 512, original_pixels)), (0, 0))
        ImageDraw.Draw(mask).rectangle([0, original_pixels, 512, original_pixels + inpaint_pixels], fill=255)
    elif direction == 'down':
        new_image.paste(image.crop((0, 512 - original_pixels, 512, 512)), (0, 512 - original_pixels))
        ImageDraw.Draw(mask).rectangle([0, white_pixels, 512, 512 - original_pixels], fill=255)
    else:
        raise ValueError("Invalid direction. Choose 'left', 'right', 'up', or 'down'.")

    return new_image, mask


def inpaint_edge_texture(image_path, prompt, direction, original_fraction, inpaint_fraction, seed=None):
    # Load the pipeline
    # pipeline = AutoPipelineForInpainting.from_pretrained(
    #     "runwayml/stable-diffusion-inpainting", torch_dtype=torch.float16, variant="fp16"
    # )
    pipeline = StableDiffusion3InpaintPipeline.from_pretrained(
        SD3_REPO,
        text_encoder_3=None,
        tokenizer_3=None,
        torch_dtype=torch.float16,
        token=HUGGING_FACE_HUB_TOKEN,
    ).to("cuda")

    pipeline.enable_model_cpu_offload()

    # Create the edge texture and mask
    init_image, mask_image = create_edge_texture(image_path, direction, original_fraction, inpaint_fraction)


    # Set up the generator
    if seed is not None:
        generator = torch.Generator("cuda").manual_seed(seed)
    else:
        generator = torch.Generator("cuda").manual_seed(torch.randint(0, 1000000, (1,)).item())

    # Need to upscale for SD3 to 1024 from 512
    init_image = init_image.resize((1024, 1024))
    mask_image = mask_image.resize((1024, 1024))

    # save the mask image
    mask_image.save("edge_texture_mask.png")

    # save the init image
    init_image.save("edge_texture_init.png")

    # Generate the inpainted image
    image = pipeline(
        prompt=prompt,
        image=init_image,
        mask_image=mask_image,
        generator=generator,
        guidance_scale=7.0,
    ).images[0]

    # Save the inpainted image
    image.save("edge_texture_output.png")

    return image


# Example usage
if __name__ == "__main__":
    init_image_path = "./terrain_tiles_photo/46_summer_sunny_desert.png"
    prompt = "A sunny desert landscape viewed from above. Realistic, bold exaggerated styles. Gradually disperses to reveal a white background, no sharp edges."
    direction = "left"
    seed = 92

    original_fraction = 0.3  # How much of the original tile will remain
    inpaint_fraction = 0.3  # How much of the tile will be inpainted (rest is white)

    inpainted_image = inpaint_edge_texture(init_image_path, prompt, direction, original_fraction, inpaint_fraction, seed)
    print("Inpainting complete.")
