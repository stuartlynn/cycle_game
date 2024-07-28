import numpy as np
from PIL import Image
import argparse

TAPER_RATE = 2.0


def create_gradient_mask(size, direction, taper_rate=TAPER_RATE):

    """Create a gradient mask with corrected directions and improved smoothness."""
    if direction in ['left', 'right']:
        gradient = np.linspace(0, 1, size[0])
        mask = np.tile(gradient, (size[1], 1))
    elif direction in ['top', 'bottom']:
        gradient = np.linspace(0, 1, size[1])
        mask = np.tile(gradient, (size[0], 1)).T
    elif direction in ['top_left', 'top_right', 'bottom_left', 'bottom_right']:

        x_direction = 'left' if 'left' in direction else 'right'
        x_mask = create_gradient_mask(size, x_direction, taper_rate=taper_rate)

        y_direction = 'top' if 'top' in direction else 'bottom'
        y_mask = create_gradient_mask(size, y_direction, taper_rate=taper_rate)

        # Combine x and y masks but need them back as floats first
        x_mask = x_mask.astype(float) / 255
        y_mask = y_mask.astype(float) / 255

        combined_mask = x_mask * y_mask

        return (combined_mask * 255).astype(np.uint8)

    # Apply taper rate for smoother transition
    mask = np.power(mask, taper_rate)

    # Correct the gradient direction
    if direction in ['left', 'top']:
        # Flip orientation of the matrix
        mask = np.flip(mask, axis=0 if direction == 'top' else 1)

    return (mask * 255).astype(np.uint8)


def apply_gradient_mask(image, mask):
    """Apply gradient mask to image's alpha channel."""
    rgba = np.array(image.convert('RGBA'))
    rgba[..., 3] = mask
    return Image.fromarray(rgba)


def generate_tileset(base_tile_path, output_path, taper_rate=TAPER_RATE, output_format='auto'):
    """Generate a complete 3x3 tileset with fading edges and corners."""
    base_tile = Image.open(base_tile_path).convert('RGBA')
    size = base_tile.size

    # Create new image for the tileset (3x3 grid of tiles)
    tileset = Image.new('RGBA', (size[0] * 3, size[1] * 3), (0, 0, 0, 0))

    # Place base tile in the center
    tileset.paste(base_tile, (size[0], size[1]))

    # Generate and place edge tiles
    for direction in ['left', 'right', 'top', 'bottom']:
        mask = create_gradient_mask(size, direction, taper_rate)
        edge_tile = apply_gradient_mask(base_tile, mask)

        if direction == 'right':
            tileset.paste(edge_tile, (0, size[1]), edge_tile)
        elif direction == 'left':
            tileset.paste(edge_tile, (size[0] * 2, size[1]), edge_tile)
        elif direction == 'bottom':
            tileset.paste(edge_tile, (size[0], 0), edge_tile)
        elif direction == 'top':
            tileset.paste(edge_tile, (size[0], size[1] * 2), edge_tile)

    # Generate and place corner tiles
    for direction in ['top_left', 'top_right', 'bottom_left', 'bottom_right']:
        mask = create_gradient_mask(size, direction, taper_rate)
        corner_tile = apply_gradient_mask(base_tile, mask)

        if direction == 'bottom_right':
            tileset.paste(corner_tile, (0, 0), corner_tile)
        elif direction == 'bottom_left':
            tileset.paste(corner_tile, (size[0] * 2, 0), corner_tile)
        elif direction == 'top_right':
            tileset.paste(corner_tile, (0, size[1] * 2), corner_tile)
        elif direction == 'top_left':
            tileset.paste(corner_tile, (size[0] * 2, size[1] * 2), corner_tile)

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


# Test code
if __name__ == "__main__":
    default_base_tile = "terrain_tiles_oil/44_summer_sunny_pebbles.png"
    taper_power = 2  # 0 if we want just to view the tiling

    parser = argparse.ArgumentParser(description="Generate a tileset with fading edges.")
    parser.add_argument("--base_tile",  default=default_base_tile, help="Path to the base tile image")
    parser.add_argument("--output", default="tiled_pebbles.png",  help="Path for the output tileset")
    parser.add_argument("--taper", type=float, default=taper_power, help="Taper rate for fading (default: 1.0)")
    parser.add_argument("--format", choices=['auto', 'png', 'webp'], default='auto',
                        help="Output format (default: auto)")

    args = parser.parse_args()

    generate_tileset(args.base_tile, args.output, args.taper, args.format)