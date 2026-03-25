"""
Generate placeholder sprite assets for the Galaga clone.
All sprites are 16x16 colored placeholder tiles.

Sprite atlas layout (enemies.png, 128x64 = 8 cols x 4 rows of 16x16):
  Row 0: Boss G1, Boss G2, Boss P1, Boss P2, Butterfly1, Butterfly2, Bee1, Bee2
  Row 1: Scorpion1, Scorpion2, Stingray1, Stingray2, Flagship, Flag1, Flag5, Flag10
  Row 2: Expl1, Expl2, Expl3, Expl4, Expl5, Expl6, Beam1, Beam2
  Row 3: Beam3, Beam4, Flag20, Flag30, Flag50, (empty x3)
"""

import struct
import zlib
import os


def create_png(width, height, pixels):
    """Create PNG bytes from RGBA pixel data (flat list of (r,g,b,a) tuples, row-major)."""
    def chunk(name, data):
        c = name + data
        return struct.pack(">I", len(data)) + c + struct.pack(">I", zlib.crc32(c) & 0xFFFFFFFF)

    sig = b"\x89PNG\r\n\x1a\n"
    ihdr = chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))

    raw = bytearray()
    for y in range(height):
        raw += b"\x00"  # filter type None
        for x in range(width):
            r, g, b, a = pixels[y * width + x]
            raw += bytes([r, g, b, a])

    idat = chunk(b"IDAT", zlib.compress(bytes(raw)))
    iend = chunk(b"IEND", b"")
    return sig + ihdr + idat + iend


def save_png(path, width, height, pixels):
    os.makedirs(os.path.dirname(path) if os.path.dirname(path) else ".", exist_ok=True)
    data = create_png(width, height, pixels)
    with open(path, "wb") as f:
        f.write(data)
    print(f"  Generated {path} ({width}x{height})")


def make_canvas(width, height, fill=(0, 0, 0, 0)):
    return [fill] * (width * height)


def draw_rect(pixels, width, x0, y0, w, h, color):
    for y in range(y0, y0 + h):
        for x in range(x0, x0 + w):
            pixels[y * width + x] = color


def draw_border(pixels, width, x0, y0, w, h, color):
    for x in range(x0, x0 + w):
        pixels[y0 * width + x] = color
        pixels[(y0 + h - 1) * width + x] = color
    for y in range(y0, y0 + h):
        pixels[y * width + x0] = color
        pixels[y * width + (x0 + w - 1)] = color


def draw_sprite_tile(pixels, atlas_width, col, row, fill, border, dot=None):
    """Draw a 16x16 sprite tile at grid position (col, row)."""
    x0 = col * 16
    y0 = row * 16
    # Fill interior (14x14)
    draw_rect(pixels, atlas_width, x0 + 1, y0 + 1, 14, 14, fill)
    # Border
    draw_border(pixels, atlas_width, x0, y0, 16, 16, border)
    # Optional center dot
    if dot:
        draw_rect(pixels, atlas_width, x0 + 6, y0 + 6, 4, 4, dot)


def make_explosion_color(frame):
    """Explosions fade from bright yellow-white to dark red over 6 frames."""
    stages = [
        ((255, 255, 200, 255), (255, 255, 100, 255)),
        ((255, 220, 100, 255), (255, 200, 50, 255)),
        ((255, 180, 50, 255), (255, 150, 20, 255)),
        ((220, 120, 20, 255), (200, 100, 10, 255)),
        ((160, 70, 10, 255), (140, 60, 5, 255)),
        ((100, 40, 5, 255), (80, 30, 2, 255)),
    ]
    return stages[min(frame, 5)]


def gen_enemies_atlas():
    W, H = 128, 64
    pixels = make_canvas(W, H)

    # ---- Row 0: Main enemy types ----
    # Boss Galaga green (frame 1, 2)
    draw_sprite_tile(pixels, W, 0, 0, (0, 180, 0, 255), (0, 255, 0, 255), dot=(0, 255, 100, 255))
    draw_sprite_tile(pixels, W, 1, 0, (0, 160, 0, 255), (0, 230, 0, 255), dot=(0, 200, 80, 255))
    # Boss Galaga purple (frame 1, 2 — after 1st hit)
    draw_sprite_tile(pixels, W, 2, 0, (140, 0, 200, 255), (200, 0, 255, 255), dot=(180, 0, 255, 255))
    draw_sprite_tile(pixels, W, 3, 0, (120, 0, 180, 255), (180, 0, 240, 255), dot=(160, 0, 240, 255))
    # Butterfly (red/white, frames 1 & 2)
    draw_sprite_tile(pixels, W, 4, 0, (200, 0, 0, 255), (255, 80, 80, 255), dot=(255, 200, 200, 255))
    draw_sprite_tile(pixels, W, 5, 0, (180, 0, 0, 255), (230, 60, 60, 255), dot=(230, 180, 180, 255))
    # Bee (blue body / yellow accent, frames 1 & 2)
    draw_sprite_tile(pixels, W, 6, 0, (0, 80, 200, 255), (255, 255, 0, 255), dot=(255, 220, 0, 255))
    draw_sprite_tile(pixels, W, 7, 0, (0, 60, 180, 255), (230, 230, 0, 255), dot=(230, 200, 0, 255))

    # ---- Row 1: Splitters + Flagship + stage flags ----
    # Scorpion splitter (yellow, frames 1 & 2)
    draw_sprite_tile(pixels, W, 0, 1, (200, 200, 0, 255), (255, 255, 50, 255), dot=(255, 255, 150, 255))
    draw_sprite_tile(pixels, W, 1, 1, (180, 180, 0, 255), (240, 240, 40, 255), dot=(240, 240, 130, 255))
    # Stingray splitter (green, frames 1 & 2)
    draw_sprite_tile(pixels, W, 2, 1, (0, 180, 80, 255), (0, 255, 120, 255), dot=(100, 255, 180, 255))
    draw_sprite_tile(pixels, W, 3, 1, (0, 160, 70, 255), (0, 230, 100, 255), dot=(80, 230, 160, 255))
    # Galaxian Flagship (orange)
    draw_sprite_tile(pixels, W, 4, 1, (200, 100, 0, 255), (255, 150, 0, 255), dot=(255, 200, 100, 255))
    # Stage flags: 1, 5, 10
    draw_sprite_tile(pixels, W, 5, 1, (80, 80, 255, 255), (150, 150, 255, 255))
    draw_sprite_tile(pixels, W, 6, 1, (80, 200, 80, 255), (150, 255, 150, 255))
    draw_sprite_tile(pixels, W, 7, 1, (200, 80, 80, 255), (255, 150, 150, 255))

    # ---- Row 2: Explosions (6 frames) + Beam frames 1-2 ----
    for i in range(6):
        fill, border = make_explosion_color(i)
        draw_sprite_tile(pixels, W, i, 2, fill, border)
    # Tractor beam frames 1 & 2 (cyan)
    draw_sprite_tile(pixels, W, 6, 2, (0, 200, 200, 200), (0, 255, 222, 255))
    draw_sprite_tile(pixels, W, 7, 2, (0, 180, 180, 180), (0, 230, 200, 255))

    # ---- Row 3: Beam frames 3-4 + remaining stage flags ----
    draw_sprite_tile(pixels, W, 0, 3, (0, 160, 160, 160), (0, 210, 180, 255))
    draw_sprite_tile(pixels, W, 1, 3, (0, 140, 140, 140), (0, 190, 160, 255))
    # Stage flags: 20, 30, 50
    draw_sprite_tile(pixels, W, 2, 3, (200, 200, 200, 255), (255, 255, 255, 255))
    draw_sprite_tile(pixels, W, 3, 3, (180, 180, 220, 255), (220, 220, 255, 255))
    draw_sprite_tile(pixels, W, 4, 3, (220, 180, 180, 255), (255, 220, 220, 255))
    # Slots 5-7 remain empty (transparent)

    save_png("assets/sprites/enemies.png", W, H, pixels)


def gen_player():
    W, H = 16, 16
    pixels = make_canvas(W, H)
    # Simple triangular ship silhouette (pointing up)
    white = (222, 222, 222, 255)
    shape = [
        "       ##       ",
        "      ####      ",
        "      ####      ",
        "     ######     ",
        "     ######     ",
        "    ########    ",
        "    ########    ",
        "   ##########   ",
        "  ############  ",
        " ###############",  # Note: 17 chars so trim
        "################",
        "################",
        "################",
        " ##############",
        "  ## ######## ##",
        "     ########   ",
    ]
    # Simpler approach: direct pixel drawing
    pixels = make_canvas(W, H)
    ship_pixels = [
        (7, 0), (8, 0),
        (6, 1), (7, 1), (8, 1), (9, 1),
        (6, 2), (7, 2), (8, 2), (9, 2),
        (5, 3), (6, 3), (7, 3), (8, 3), (9, 3), (10, 3),
        (5, 4), (6, 4), (7, 4), (8, 4), (9, 4), (10, 4),
        (4, 5), (5, 5), (6, 5), (7, 5), (8, 5), (9, 5), (10, 5), (11, 5),
        (3, 6), (4, 6), (5, 6), (6, 6), (7, 6), (8, 6), (9, 6), (10, 6), (11, 6), (12, 6),
        (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7), (9, 7), (10, 7), (11, 7), (12, 7), (13, 7),
        (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), (8, 8), (9, 8), (10, 8), (11, 8), (12, 8), (13, 8), (14, 8),
        (0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9), (6, 9), (7, 9), (8, 9), (9, 9), (10, 9), (11, 9), (12, 9), (13, 9), (14, 9), (15, 9),
        (0, 10), (1, 10), (2, 10), (3, 10), (4, 10), (5, 10), (6, 10), (7, 10), (8, 10), (9, 10), (10, 10), (11, 10), (12, 10), (13, 10), (14, 10), (15, 10),
        (0, 11), (1, 11), (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11), (8, 11), (9, 11), (10, 11), (11, 11), (12, 11), (13, 11), (14, 11), (15, 11),
        (0, 12), (1, 12), (2, 12), (3, 12), (4, 12), (5, 12), (6, 12), (7, 12), (8, 12), (9, 12), (10, 12), (11, 12), (12, 12), (13, 12), (14, 12), (15, 12),
        (1, 13), (2, 13), (4, 13), (5, 13), (6, 13), (7, 13), (8, 13), (9, 13), (10, 13), (11, 13), (13, 13), (14, 13),
        (4, 14), (5, 14), (6, 14), (7, 14), (8, 14), (9, 14), (10, 14), (11, 14),
        (5, 15), (6, 15), (7, 15), (8, 15), (9, 15), (10, 15),
    ]
    for x, y in ship_pixels:
        if 0 <= x < W and 0 <= y < H:
            pixels[y * W + x] = white
    save_png("assets/sprites/player.png", W, H, pixels)


def gen_bullets():
    """Two bullet sprites stacked vertically: player (white) and enemy (red), each 4x8."""
    W, H = 4, 16
    pixels = make_canvas(W, H)
    white = (222, 222, 222, 255)
    red = (255, 80, 80, 255)
    # Player bullet (top 8 rows): thin vertical bar
    for y in range(1, 7):
        pixels[y * W + 1] = white
        pixels[y * W + 2] = white
    # Enemy bullet (bottom 8 rows): thin vertical bar, red
    for y in range(9, 15):
        pixels[y * W + 1] = red
        pixels[y * W + 2] = red
    save_png("assets/sprites/bullets.png", W, H, pixels)


def gen_tractor_beam():
    """4 frames of tractor beam animation (fan shape, cyan), each 16x16, stacked vertically."""
    W, H = 16, 64
    pixels = make_canvas(W, H)
    for frame in range(4):
        y0 = frame * 16
        alpha = 200 - frame * 30
        for y in range(16):
            # Fan widens as y increases, also varies by frame
            fan = (frame + 1) * 2 + y // 3
            cx = 8
            for x in range(16):
                if abs(x - cx) <= fan:
                    brightness = max(100, 200 - abs(x - cx) * 15)
                    pixels[(y0 + y) * W + x] = (0, brightness, brightness, alpha)
    save_png("assets/sprites/tractor_beam.png", W, H, pixels)


def gen_lives_icon():
    """Mini player ship icon for the lives HUD (16x16)."""
    W, H = 16, 16
    pixels = make_canvas(W, H)
    white = (222, 222, 222, 255)
    # Simplified 8x8 ship centered in 16x16
    ship_pixels = [
        (7, 2), (8, 2),
        (6, 3), (7, 3), (8, 3), (9, 3),
        (5, 4), (6, 4), (7, 4), (8, 4), (9, 4), (10, 4),
        (4, 5), (5, 5), (6, 5), (7, 5), (8, 5), (9, 5), (10, 5), (11, 5),
        (3, 6), (4, 6), (5, 6), (6, 6), (7, 6), (8, 6), (9, 6), (10, 6), (11, 6), (12, 6),
        (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7), (8, 7), (9, 7), (10, 7), (11, 7), (12, 7), (13, 7),
        (2, 8), (3, 8), (4, 8), (5, 8), (6, 8), (7, 8), (8, 8), (9, 8), (10, 8), (11, 8), (12, 8), (13, 8),
        (3, 9), (4, 9), (6, 9), (7, 9), (8, 9), (9, 9), (11, 9), (12, 9),
    ]
    for x, y in ship_pixels:
        if 0 <= x < W and 0 <= y < H:
            pixels[y * W + x] = white
    save_png("assets/sprites/ui/lives_icon.png", W, H, pixels)


def gen_stage_flags():
    """Stage flag icons: 1, 5, 10, 20, 30, 50 - each 8x8, arranged horizontally in a strip."""
    W, H = 48, 8
    pixels = make_canvas(W, H)
    colors = [
        (100, 100, 255, 255),   # 1  - blue
        (100, 200, 100, 255),   # 5  - green
        (200, 100, 100, 255),   # 10 - red
        (200, 200, 100, 255),   # 20 - yellow
        (100, 200, 200, 255),   # 30 - cyan
        (200, 100, 200, 255),   # 50 - magenta
    ]
    for i, color in enumerate(colors):
        x0 = i * 8
        for y in range(8):
            for x in range(8):
                if x == 0 or x == 7 or y == 0 or y == 7:
                    pixels[y * W + (x0 + x)] = tuple(max(0, min(255, c + 50)) if j < 3 else c for j, c in enumerate(color))
                else:
                    pixels[y * W + (x0 + x)] = color
    save_png("assets/sprites/stage_flags.png", W, H, pixels)


if __name__ == "__main__":
    print("Generating placeholder sprite assets...")
    gen_enemies_atlas()
    gen_player()
    gen_bullets()
    gen_tractor_beam()
    gen_lives_icon()
    gen_stage_flags()
    print("Done! All placeholder sprites created.")
    print()
    print("NOTE: These are colored placeholder sprites.")
    print("Replace with actual pixel art before final release.")
