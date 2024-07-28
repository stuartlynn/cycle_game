""" Create terrain tiles over a mix of conditions & seasons."""

import os
import torch
from PIL.Image import Resampling
from diffusers import StableDiffusionXLPipeline

from genai_assets.tiling import seamless_tiling

SEASONS = ["spring", "summer", "autumn", "winter"]
# CONDITIONS = ["sunny", "night"] # probably just rely on in-game lighting
SURFACE_TYPES = ["grass", "desert", "rock", "ocean", "pebbles", "dirt"]

SEED = 43
RESOLUTION = 512  # Default for sdxl turbo

OUTPUT_RESOLUTION = 128  # Create images on 128x128 resolution

OUTPUT_FOLDER = "terrain_tiles_oils"
N_STEPS = 2
GUIDANCE = 0.0

SDXL_TURBO = "stabilityai/sdxl-turbo"

os.makedirs(OUTPUT_FOLDER, exist_ok=True)


def make_seamless_pipeline():
    """Make a seamless pipeline """

    pipe = StableDiffusionXLPipeline.from_pretrained(
        SDXL_TURBO, torch_dtype=torch.float16
    ).to("cuda")
    pipe.enable_model_cpu_offload()

    return seamless_tiling(pipeline=pipe)


def create_terrain_tile(pipeline, surface: str, season: str, condition = None) -> str:
    """Create a terrain tile over a mix of conditions & seasons."""

#    prompt = f"Texture of {surface} during {season} season while it is {condition}. As viewed from above, at a medium distance. Surreal oil painting, bold exaggerated styles."
#     prompt = f"Very simple plain texture of {surface} during {season} season while it is {condition}. As viewed from above, at a medium distance. Realistic, bold exaggerated styles."
    prompt = f"Very simple plain texture of {surface} during {season} season.  As viewed from above, at a medium distance. Surreal oil painting, bold exaggerated styles."

    if condition is not None:
        filename = f"{SEED}_{season}_{condition}_{surface}.png"
    else:
        filename = f"{SEED}_{season}_{surface}.png"

    image = make_tiled_image(prompt, pipeline, seed=SEED)

    path = os.path.join(OUTPUT_FOLDER, filename)

    # Downsample the image to 128x128
    image = image.resize((OUTPUT_RESOLUTION, OUTPUT_RESOLUTION), resample=Resampling.LANCZOS)

    image.save(path)


def make_tiled_image(prompt, pipeline, seed=42):
    """Make a tiled image."""

    generator = torch.Generator().manual_seed(seed)

    image = pipeline(
        prompt=prompt,
        generator=generator,
        num_inference_steps=N_STEPS,
        guidance_scale=GUIDANCE,
        height=RESOLUTION,
        width=RESOLUTION,
    ).images[0]

    return image


if __name__ == "__main__":

    pipe = make_seamless_pipeline()

    for season in SEASONS:
        # for condition in CONDITIONS:
        for surface in SURFACE_TYPES:
            create_terrain_tile(pipe, surface, season)
