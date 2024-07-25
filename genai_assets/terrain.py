""" Create terrain tiles over a mix of conditions & seasons."""

import os

import torch
from diffusers import StableDiffusionXLPipeline

from genai_assets.tiling import seamless_tiling

SEASONS = ["spring", "summer", "fall", "winter"]
CONDITIONS = ["sunny", "rainy", "night"]
SURFACE_TYPES = ["grass", "sand", "rock", "water"]
SEED = 42
OUTPUT_FOLDER = "terrain_tiles"
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
    pipe = seamless_tiling(pipeline=pipe, x_axis=True, y_axis=True)
    return pipe


def create_terrain_tile(pipeline, surface: str, season: str, condition: str) -> str:
    """Create a terrain tile over a mix of conditions & seasons."""

    prompt = f"Texture of {surface} during {season} season while it is {condition}."

    filename = f"{SEED}_{season}_{condition}_{surface}.png"
    folder = "terrain_tiles"

    image = make_tiled_image(prompt, pipeline, seed=SEED)

    path = os.path.join(folder, filename)
    image.save(path)


def make_tiled_image(prompt, pipeline, seed=42):
    """Make a tiled image."""

    generator = torch.Generator().manual_seed(seed)

    image = pipeline(
        prompt=prompt,
        generator=generator,
        num_inference_steps=N_STEPS,
        guidance_scale=GUIDANCE,
        height=512,
        width=512,
    ).images[0]

    return image


if __name__ == "__main__":

    pipe = make_seamless_pipeline()

    for season in SEASONS:
        for condition in CONDITIONS:
            for surface in SURFACE_TYPES:
                create_terrain_tile(pipe, surface, season, condition)
