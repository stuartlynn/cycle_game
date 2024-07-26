""" Make a full set of tildes from a single central tile. """
import numpy as np
from PIL import Image
import argparse


def create_gradient_mask(size, direction, taper_rate=1.0):
    """Create a gradient mask with configurable taper rate."""
    if direction in ['left', 'right']:
        gradient = np.linspace(0, 1, size[0])
        mask = np.tile(gradient, (size[1], 1))
    elif direction in ['top', 'bottom']:
        gradient = np.linspace(0, 1, size[1])
        mask = np.tile(gradient, (size[0], 1)).T
    elif direction in ['top_left', 'top_right', 'bottom_left', 'bottom_right']:
        x = np.linspace(0, 1, size[0])
        y = np.linspace(0, 1, size[1])
        xx, yy = np.meshgrid(x, y)
        mask = np.sqrt(xx ** 2 + yy ** 2)
        mask = (mask - mask.min()) / (mask.max() - mask.min())

    # Apply taper rate
    mask = np.power(mask, taper_rate)

    if direction in ['left', 'top', 'top_left']:
        mask = 1 - mask

    return (mask * 255).astype(np.uint8)


def apply_gradient_mask(image, mask):
    """Apply gradient mask to image."""
    rgba = np.array(image.convert('RGBA'))
    rgba[..., 3] = mask
    return Image.fromarray(rgba)


def generate_tileset(base_tile_path, output_path, taper_rate=1.0, output_format='auto'):
    """Generate a complete tileset with fading edges and corners."""
    base_tile = Image.open(base_tile_path).convert('RGBA')
    size = base_tile.size

    # Create new image for the tileset (4x4 grid of tiles to include base tile)
    tileset = Image.new('RGBA', (size[0] * 4, size[1] * 4), (0, 0, 0, 0))

    # Place base tiles
    for i in range(4):
        for j in range(4):
            tileset.paste(base_tile, (size[0] * i, size[1] * j))

    # Generate and place edge tiles
    for direction in ['left', 'right', 'top', 'bottom']:
        mask = create_gradient_mask(size, direction, taper_rate)
        edge_tile = apply_gradient_mask(base_tile, mask)

        if direction == 'left':
            for i in range(4):
                tileset.paste(edge_tile, (0, size[1] * i), edge_tile)
        elif direction == 'right':
            for i in range(4):
                tileset.paste(edge_tile, (size[0] * 3, size[1] * i), edge_tile)
        elif direction == 'top':
            for i in range(4):
                tileset.paste(edge_tile, (size[0] * i, 0), edge_tile)
        elif direction == 'bottom':
            for i in range(4):
                tileset.paste(edge_tile, (size[0] * i, size[1] * 3), edge_tile)

    # Generate and place corner tiles
    for direction in ['top_left', 'top_right', 'bottom_left', 'bottom_right']:
        mask = create_gradient_mask(size, direction, taper_rate)
        corner_tile = apply_gradient_mask(base_tile, mask)

        if direction == 'top_left':
            tileset.paste(corner_tile, (0, 0), corner_tile)
        elif direction == 'top_right':
            tileset.paste(corner_tile, (size[0] * 3, 0), corner_tile)
        elif direction == 'bottom_left':
            tileset.paste(corner_tile, (0, size[1] * 3), corner_tile)
        elif direction == 'bottom_right':
            tileset.paste(corner_tile, (size[0] * 3, size[1] * 3), corner_tile)

    # Determine output format
    if output_format == 'auto':
        output_format = output_path.split('.')[-1].lower()

    # Save the tileset
    if output_format == 'webp':
        tileset.save(output_path, 'WEBP', lossless=True, quality=100)
    elif output_format == 'png':
        tileset.save(output_path, 'PNG')
    else:
        raise ValueError(f"Unsupported output format: {output_format}")

    print(f"Tileset saved to {output_path}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a tileset with fading edges.")
    parser.add_argument("base_tile", help="Path to the base tile image")
    parser.add_argument("output", help="Path for the output tileset")
    parser.add_argument("--taper", type=float, default=1.0, help="Taper rate for fading (default: 1.0)")
    parser.add_argument("--format", choices=['auto', 'png', 'webp'], default='auto',
                        help="Output format (default: auto)")

    args = parser.parse_args()

    generate_tileset(args.base_tile, args.output, args.taper, args.format)