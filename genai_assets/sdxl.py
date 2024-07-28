""" Mass production of SDXL files from a template. """

import os

from diffusers import StableDiffusionXLPipeline
import torch

DEBUG = True # just make a single image
RANDOM_SEED = 42
N_STEPS = 2
N_SEEDS = 3
CPU_OFFLOAD = False
CREATURE = "cow"
COLOUR = "purple"
OUTPUT_FOLDER = f"{CREATURE}_images"
SDXL_TURBO = "stabilityai/sdxl-turbo"

os.makedirs(OUTPUT_FOLDER, exist_ok=True)

pipe = StableDiffusionXLPipeline.from_pretrained(
    SDXL_TURBO, torch_dtype=torch.float16
).to("cuda")
pipe.enable_model_cpu_offload()

human_prompt =

base_prompt = f"{COLOUR} {CREATURE} Garkactigaca, side view, 3d render, profile view, dynamic, angry green eyes, tufts of hair, long tail, pixar, disney, nemo, high quality, full shot, on a white background, "
base_prompt = human_prompt

def make_image(name, image_prompt):

    generator = torch.Generator().manual_seed(RANDOM_SEED)

    image = pipe(
        prompt=image_prompt,
        generator=generator,
        num_inference_steps=N_STEPS,
        guidance_scale=0.0,
        height=512,
        width=512,
    ).images[0]

    path = os.path.join(OUTPUT_FOLDER, f"{name}.png")
    image.save(path)


# make poses which are comprised of action and view
prompts = []
filenames = []

if DEBUG:
    actions = ["moving"]
    views = ["side"]
    N_SEEDS = 1
    N_STEPS = 1
else:
    actions = ["moving", "running", "walking", "jumping", "sitting", "flying", "leaping", "playing"]
    views = ["side", "front",  "back", "top", "bottom", "isometric", "orthographic", "perspective"]


for action in actions:
    for view in views:
        if DEBUG:
            full_prompt = f"A grassy field"
        else:
            full_prompt = f"{action} {base_prompt}, camera {view} view perspective"

        # If pose contains "back view", must remove "green eyes" from prompt or else it forces eyes on the back of the head
        if "back view" in full_prompt:
            full_prompt = full_prompt.replace("angry green eyes,", "")

        prompts.append(full_prompt)
        filenames.append(f"{RANDOM_SEED}_{action}_{view}")

for p, name in zip(prompts, filenames):
    make_image(name, p)

