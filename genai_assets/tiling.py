import torch
from typing import Optional
from diffusers import StableDiffusionPipeline
from diffusers.models.lora import LoRACompatibleConv


def seamless_tiling(pipeline, x_axis: bool = True, y_axis: bool = True):
    def asymmetric_conv2d_convforward(self, input: torch.Tensor, weight: torch.Tensor, bias: Optional[torch.Tensor] = None):
        self.paddingX = (self._reversed_padding_repeated_twice[0], self._reversed_padding_repeated_twice[1], 0, 0)
        self.paddingY = (0, 0, self._reversed_padding_repeated_twice[2], self._reversed_padding_repeated_twice[3])
        working = torch.nn.functional.pad(input, self.paddingX, mode=x_mode)
        working = torch.nn.functional.pad(working, self.paddingY, mode=y_mode)
        return torch.nn.functional.conv2d(working, weight, bias, self.stride, torch.nn.modules.utils._pair(0), self.dilation, self.groups)
    x_mode = 'circular' if x_axis else 'constant'
    y_mode = 'circular' if y_axis else 'constant'
    targets = [pipeline.vae, pipeline.text_encoder, pipeline.unet]
    convolution_layers = []
    for target in targets:
        for module in target.modules():
            if isinstance(module, torch.nn.Conv2d):
                convolution_layers.append(module)
    for layer in convolution_layers:
        if isinstance(layer, LoRACompatibleConv) and layer.lora_layer is None:
            layer.lora_layer = lambda * x: 0
        layer._conv_forward = asymmetric_conv2d_convforward.__get__(layer, torch.nn.Conv2d)
    return pipeline




# pipeline = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5", torch_dtype=torch.float16, use_safetensors=True)
# pipeline.enable_model_cpu_offload()
# prompt = ["texture of a red brick wall"]
# seed = 123456
# generator = torch.Generator(device='cuda').manual_seed(seed)
#
# pipeline = seamless_tiling(pipeline=pipeline, x_axis=True, y_axis=True)
# image = pipeline(
#     prompt=prompt,
#     width=512,
#     height=512,
#     num_inference_steps=20,
#     guidance_scale=7,
#     num_images_per_prompt=1,
#     generator=generator
# ).images[0]
# image.save('tiled_image.png')


# seamless_tiling(pipeline=pipeline, x_axis=False, y_axis=False)
#
# torch.cuda.empty_cache()
