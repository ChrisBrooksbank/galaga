"""
Generate Galaga-style pixel art sprites using Pillow.
Outputs to assets/sprites/
"""

from PIL import Image, ImageDraw
import os

OUT = os.path.join(os.path.dirname(__file__), "assets", "sprites")
os.makedirs(OUT, exist_ok=True)

# Color palette (classic Galaga)
TRANSPARENT = (0, 0, 0, 0)
WHITE = (255, 255, 255, 255)
BLUE = (0, 100, 255, 255)
LIGHT_BLUE = (100, 180, 255, 255)
CYAN = (0, 255, 255, 255)
DARK_BLUE = (0, 50, 180, 255)
RED = (255, 0, 0, 255)
DARK_RED = (180, 0, 0, 255)
YELLOW = (255, 255, 0, 255)
ORANGE = (255, 165, 0, 255)
DARK_ORANGE = (200, 100, 0, 255)
GREEN = (0, 200, 0, 255)
DARK_GREEN = (0, 140, 0, 255)
LIGHT_GREEN = (100, 255, 100, 255)
PURPLE = (180, 0, 255, 255)
DARK_PURPLE = (120, 0, 180, 255)
LIGHT_PURPLE = (220, 100, 255, 255)
GRAY = (180, 180, 180, 255)


def px(img, x, y, color):
    """Set a pixel if in bounds."""
    if 0 <= x < img.width and 0 <= y < img.height:
        img.putpixel((x, y), color)


def mirror_h(pixels):
    """Given list of (x, y, color) for left half, mirror horizontally around center (7.5 for 16px)."""
    result = list(pixels)
    for x, y, c in pixels:
        mx = 15 - x
        if mx != x:
            result.append((mx, y, c))
    return result


# =============================================================================
# 1. PLAYER SHIP (16x16)
# =============================================================================
def make_player():
    img = Image.new("RGBA", (16, 16), TRANSPARENT)
    # Classic Galaga player: sleek upward-pointing fighter
    # Build left half then mirror
    pixels = [
        # Nose (top center)
        (7, 1, WHITE), (8, 1, WHITE),
        (7, 2, WHITE), (8, 2, WHITE),
        # Upper fuselage
        (6, 3, LIGHT_BLUE), (7, 3, WHITE), (8, 3, WHITE), (9, 3, LIGHT_BLUE),
        (6, 4, LIGHT_BLUE), (7, 4, WHITE), (8, 4, WHITE), (9, 4, LIGHT_BLUE),
        (5, 5, BLUE), (6, 5, LIGHT_BLUE), (7, 5, WHITE), (8, 5, WHITE), (9, 5, LIGHT_BLUE), (10, 5, BLUE),
        # Mid body
        (5, 6, BLUE), (6, 6, LIGHT_BLUE), (7, 6, WHITE), (8, 6, WHITE), (9, 6, LIGHT_BLUE), (10, 6, BLUE),
        (4, 7, BLUE), (5, 7, BLUE), (6, 7, LIGHT_BLUE), (7, 7, WHITE), (8, 7, WHITE), (9, 7, LIGHT_BLUE), (10, 7, BLUE), (11, 7, BLUE),
        # Wings start spreading
        (3, 8, BLUE), (4, 8, BLUE), (5, 8, LIGHT_BLUE), (6, 8, WHITE), (7, 8, WHITE), (8, 8, WHITE), (9, 8, WHITE), (10, 8, LIGHT_BLUE), (11, 8, BLUE), (12, 8, BLUE),
        (2, 9, DARK_BLUE), (3, 9, BLUE), (4, 9, LIGHT_BLUE), (5, 9, LIGHT_BLUE), (6, 9, WHITE), (7, 9, WHITE), (8, 9, WHITE), (9, 9, WHITE), (10, 9, LIGHT_BLUE), (11, 9, LIGHT_BLUE), (12, 9, BLUE), (13, 9, DARK_BLUE),
        # Wide wings
        (1, 10, DARK_BLUE), (2, 10, BLUE), (3, 10, BLUE), (4, 10, LIGHT_BLUE), (5, 10, WHITE), (6, 10, WHITE), (7, 10, GRAY), (8, 10, GRAY), (9, 10, WHITE), (10, 10, WHITE), (11, 10, LIGHT_BLUE), (12, 10, BLUE), (13, 10, BLUE), (14, 10, DARK_BLUE),
        (1, 11, DARK_BLUE), (2, 11, BLUE), (3, 11, LIGHT_BLUE), (4, 11, WHITE), (5, 11, WHITE), (6, 11, GRAY), (7, 11, BLUE), (8, 11, BLUE), (9, 11, GRAY), (10, 11, WHITE), (11, 11, WHITE), (12, 11, LIGHT_BLUE), (13, 11, BLUE), (14, 11, DARK_BLUE),
        # Lower body / engine
        (2, 12, BLUE), (3, 12, LIGHT_BLUE), (4, 12, WHITE), (5, 12, GRAY), (6, 12, BLUE), (7, 12, DARK_BLUE), (8, 12, DARK_BLUE), (9, 12, BLUE), (10, 12, GRAY), (11, 12, WHITE), (12, 12, LIGHT_BLUE), (13, 12, BLUE),
        (3, 13, BLUE), (4, 13, LIGHT_BLUE), (5, 13, BLUE), (6, 13, DARK_BLUE), (7, 13, DARK_BLUE), (8, 13, DARK_BLUE), (9, 13, DARK_BLUE), (10, 13, BLUE), (11, 13, LIGHT_BLUE), (12, 13, BLUE),
        # Engine exhaust ports
        (5, 14, CYAN), (6, 14, BLUE), (7, 14, DARK_BLUE), (8, 14, DARK_BLUE), (9, 14, BLUE), (10, 14, CYAN),
        (6, 15, CYAN), (7, 15, BLUE), (8, 15, BLUE), (9, 15, CYAN),
    ]
    for x, y, c in pixels:
        px(img, x, y, c)
    img.save(os.path.join(OUT, "player.png"))
    print("  player.png")


# =============================================================================
# 2. ENEMIES SPRITE SHEET (128x64, 8 cols x 4 rows of 16x16)
# =============================================================================
def draw_boss_green_f1(img, ox, oy):
    """Boss Galaga green - frame 1: wings spread."""
    # Body center
    for y in range(3, 13):
        for x in range(6, 10):
            px(img, ox+x, oy+y, GREEN)
    # Head
    for x in range(5, 11):
        px(img, ox+x, oy+2, DARK_GREEN)
    for x in range(6, 10):
        px(img, ox+x, oy+1, GREEN)
    px(img, ox+7, oy+0, LIGHT_GREEN)
    px(img, ox+8, oy+0, LIGHT_GREEN)
    # Eyes
    px(img, ox+6, oy+3, WHITE)
    px(img, ox+9, oy+3, WHITE)
    px(img, ox+6, oy+4, YELLOW)
    px(img, ox+9, oy+4, YELLOW)
    # Wings spread
    for y in range(4, 10):
        px(img, ox+5, oy+y, GREEN)
        px(img, ox+10, oy+y, GREEN)
    for y in range(5, 9):
        px(img, ox+4, oy+y, DARK_GREEN)
        px(img, ox+11, oy+y, DARK_GREEN)
    for y in range(6, 8):
        px(img, ox+3, oy+y, GREEN)
        px(img, ox+12, oy+y, GREEN)
        px(img, ox+2, oy+y, DARK_GREEN)
        px(img, ox+13, oy+y, DARK_GREEN)
    # Wing tips
    px(img, ox+1, oy+6, LIGHT_GREEN)
    px(img, ox+1, oy+7, LIGHT_GREEN)
    px(img, ox+14, oy+6, LIGHT_GREEN)
    px(img, ox+14, oy+7, LIGHT_GREEN)
    # Lower body details
    for x in range(5, 11):
        px(img, ox+x, oy+11, DARK_GREEN)
    px(img, ox+6, oy+12, GREEN)
    px(img, ox+7, oy+12, DARK_GREEN)
    px(img, ox+8, oy+12, DARK_GREEN)
    px(img, ox+9, oy+12, GREEN)
    px(img, ox+7, oy+13, GREEN)
    px(img, ox+8, oy+13, GREEN)


def draw_boss_green_f2(img, ox, oy):
    """Boss Galaga green - frame 2: wings folded."""
    # Same body but wings closer
    for y in range(3, 13):
        for x in range(6, 10):
            px(img, ox+x, oy+y, GREEN)
    for x in range(5, 11):
        px(img, ox+x, oy+2, DARK_GREEN)
    for x in range(6, 10):
        px(img, ox+x, oy+1, GREEN)
    px(img, ox+7, oy+0, LIGHT_GREEN)
    px(img, ox+8, oy+0, LIGHT_GREEN)
    px(img, ox+6, oy+3, WHITE)
    px(img, ox+9, oy+3, WHITE)
    px(img, ox+6, oy+4, YELLOW)
    px(img, ox+9, oy+4, YELLOW)
    # Wings closer to body
    for y in range(4, 10):
        px(img, ox+5, oy+y, GREEN)
        px(img, ox+10, oy+y, GREEN)
    for y in range(5, 9):
        px(img, ox+4, oy+y, DARK_GREEN)
        px(img, ox+11, oy+y, DARK_GREEN)
    for y in range(6, 8):
        px(img, ox+3, oy+y, LIGHT_GREEN)
        px(img, ox+12, oy+y, LIGHT_GREEN)
    # Lower body
    for x in range(5, 11):
        px(img, ox+x, oy+11, DARK_GREEN)
    px(img, ox+6, oy+12, GREEN)
    px(img, ox+9, oy+12, GREEN)
    px(img, ox+7, oy+13, GREEN)
    px(img, ox+8, oy+13, GREEN)


def draw_boss_purple_f1(img, ox, oy):
    """Boss Galaga purple - frame 1."""
    for y in range(3, 13):
        for x in range(6, 10):
            px(img, ox+x, oy+y, PURPLE)
    for x in range(5, 11):
        px(img, ox+x, oy+2, DARK_PURPLE)
    for x in range(6, 10):
        px(img, ox+x, oy+1, PURPLE)
    px(img, ox+7, oy+0, LIGHT_PURPLE)
    px(img, ox+8, oy+0, LIGHT_PURPLE)
    px(img, ox+6, oy+3, WHITE)
    px(img, ox+9, oy+3, WHITE)
    px(img, ox+6, oy+4, YELLOW)
    px(img, ox+9, oy+4, YELLOW)
    for y in range(4, 10):
        px(img, ox+5, oy+y, PURPLE)
        px(img, ox+10, oy+y, PURPLE)
    for y in range(5, 9):
        px(img, ox+4, oy+y, DARK_PURPLE)
        px(img, ox+11, oy+y, DARK_PURPLE)
    for y in range(6, 8):
        px(img, ox+3, oy+y, PURPLE)
        px(img, ox+12, oy+y, PURPLE)
        px(img, ox+2, oy+y, DARK_PURPLE)
        px(img, ox+13, oy+y, DARK_PURPLE)
    px(img, ox+1, oy+6, LIGHT_PURPLE)
    px(img, ox+1, oy+7, LIGHT_PURPLE)
    px(img, ox+14, oy+6, LIGHT_PURPLE)
    px(img, ox+14, oy+7, LIGHT_PURPLE)
    for x in range(5, 11):
        px(img, ox+x, oy+11, DARK_PURPLE)
    px(img, ox+6, oy+12, PURPLE)
    px(img, ox+9, oy+12, PURPLE)
    px(img, ox+7, oy+13, PURPLE)
    px(img, ox+8, oy+13, PURPLE)


def draw_boss_purple_f2(img, ox, oy):
    """Boss Galaga purple - frame 2."""
    for y in range(3, 13):
        for x in range(6, 10):
            px(img, ox+x, oy+y, PURPLE)
    for x in range(5, 11):
        px(img, ox+x, oy+2, DARK_PURPLE)
    for x in range(6, 10):
        px(img, ox+x, oy+1, PURPLE)
    px(img, ox+7, oy+0, LIGHT_PURPLE)
    px(img, ox+8, oy+0, LIGHT_PURPLE)
    px(img, ox+6, oy+3, WHITE)
    px(img, ox+9, oy+3, WHITE)
    px(img, ox+6, oy+4, YELLOW)
    px(img, ox+9, oy+4, YELLOW)
    for y in range(4, 10):
        px(img, ox+5, oy+y, PURPLE)
        px(img, ox+10, oy+y, PURPLE)
    for y in range(5, 9):
        px(img, ox+4, oy+y, DARK_PURPLE)
        px(img, ox+11, oy+y, DARK_PURPLE)
    for y in range(6, 8):
        px(img, ox+3, oy+y, LIGHT_PURPLE)
        px(img, ox+12, oy+y, LIGHT_PURPLE)
    for x in range(5, 11):
        px(img, ox+x, oy+11, DARK_PURPLE)
    px(img, ox+6, oy+12, PURPLE)
    px(img, ox+9, oy+12, PURPLE)
    px(img, ox+7, oy+13, PURPLE)
    px(img, ox+8, oy+13, PURPLE)


def draw_butterfly_f1(img, ox, oy):
    """Butterfly enemy - frame 1: wings up."""
    # Body (thin center column)
    for y in range(2, 14):
        px(img, ox+7, oy+y, RED)
        px(img, ox+8, oy+y, RED)
    # Head
    px(img, ox+6, oy+1, BLUE)
    px(img, ox+7, oy+1, BLUE)
    px(img, ox+8, oy+1, BLUE)
    px(img, ox+9, oy+1, BLUE)
    px(img, ox+7, oy+0, LIGHT_BLUE)
    px(img, ox+8, oy+0, LIGHT_BLUE)
    # Antennae
    px(img, ox+5, oy+0, BLUE)
    px(img, ox+10, oy+0, BLUE)
    px(img, ox+6, oy+0, DARK_BLUE)
    px(img, ox+9, oy+0, DARK_BLUE)
    # Eyes
    px(img, ox+6, oy+2, WHITE)
    px(img, ox+9, oy+2, WHITE)
    # Upper wings (spread up and out)
    for dy in range(3, 8):
        dist = dy - 3
        px(img, ox+6-dist, oy+dy, BLUE)
        px(img, ox+9+dist, oy+dy, BLUE)
        if dist > 0:
            px(img, ox+6-dist+1, oy+dy, LIGHT_BLUE)
            px(img, ox+9+dist-1, oy+dy, LIGHT_BLUE)
    # Wing fill
    for dy in range(4, 7):
        for dx in range(3, 7):
            px(img, ox+dx, oy+dy, BLUE)
        for dx in range(9, 13):
            px(img, ox+dx, oy+dy, BLUE)
    # Wing highlights
    px(img, ox+4, oy+5, LIGHT_BLUE)
    px(img, ox+5, oy+5, LIGHT_BLUE)
    px(img, ox+10, oy+5, LIGHT_BLUE)
    px(img, ox+11, oy+5, LIGHT_BLUE)
    # Lower wings
    for dy in range(8, 12):
        dist = 12 - dy
        px(img, ox+6-dist, oy+dy, RED)
        px(img, ox+9+dist, oy+dy, RED)
        px(img, ox+5-dist, oy+dy, DARK_RED)
        px(img, ox+10+dist, oy+dy, DARK_RED)
    for dy in range(9, 11):
        for dx in range(4, 7):
            px(img, ox+dx, oy+dy, RED)
        for dx in range(9, 12):
            px(img, ox+dx, oy+dy, RED)
    # Tail
    px(img, ox+7, oy+13, DARK_RED)
    px(img, ox+8, oy+13, DARK_RED)
    px(img, ox+6, oy+12, RED)
    px(img, ox+9, oy+12, RED)


def draw_butterfly_f2(img, ox, oy):
    """Butterfly enemy - frame 2: wings down."""
    # Body
    for y in range(2, 14):
        px(img, ox+7, oy+y, RED)
        px(img, ox+8, oy+y, RED)
    # Head
    px(img, ox+6, oy+1, BLUE)
    px(img, ox+7, oy+1, BLUE)
    px(img, ox+8, oy+1, BLUE)
    px(img, ox+9, oy+1, BLUE)
    px(img, ox+7, oy+0, LIGHT_BLUE)
    px(img, ox+8, oy+0, LIGHT_BLUE)
    px(img, ox+5, oy+0, BLUE)
    px(img, ox+10, oy+0, BLUE)
    px(img, ox+6, oy+0, DARK_BLUE)
    px(img, ox+9, oy+0, DARK_BLUE)
    px(img, ox+6, oy+2, WHITE)
    px(img, ox+9, oy+2, WHITE)
    # Wings more folded down
    for dy in range(4, 9):
        px(img, ox+5, oy+dy, BLUE)
        px(img, ox+6, oy+dy, LIGHT_BLUE)
        px(img, ox+9, oy+dy, LIGHT_BLUE)
        px(img, ox+10, oy+dy, BLUE)
    for dy in range(5, 8):
        px(img, ox+4, oy+dy, BLUE)
        px(img, ox+11, oy+dy, BLUE)
        px(img, ox+3, oy+dy, DARK_BLUE)
        px(img, ox+12, oy+dy, DARK_BLUE)
    # Lower wings more compact
    for dy in range(9, 12):
        px(img, ox+5, oy+dy, RED)
        px(img, ox+6, oy+dy, RED)
        px(img, ox+9, oy+dy, RED)
        px(img, ox+10, oy+dy, RED)
    for dy in range(10, 12):
        px(img, ox+4, oy+dy, DARK_RED)
        px(img, ox+11, oy+dy, DARK_RED)
    px(img, ox+6, oy+12, RED)
    px(img, ox+9, oy+12, RED)
    px(img, ox+7, oy+13, DARK_RED)
    px(img, ox+8, oy+13, DARK_RED)


def draw_bee_f1(img, ox, oy):
    """Bee/Zako enemy - frame 1."""
    # Compact round body
    for x in range(5, 11):
        px(img, ox+x, oy+3, YELLOW)
        px(img, ox+x, oy+10, YELLOW)
    for y in range(4, 10):
        for x in range(4, 12):
            px(img, ox+x, oy+y, YELLOW)
    # Darker stripes
    for x in range(4, 12):
        px(img, ox+x, oy+5, DARK_ORANGE)
        px(img, ox+x, oy+7, DARK_ORANGE)
        px(img, ox+x, oy+9, DARK_ORANGE)
    # Eyes
    px(img, ox+5, oy+4, WHITE)
    px(img, ox+6, oy+4, WHITE)
    px(img, ox+9, oy+4, WHITE)
    px(img, ox+10, oy+4, WHITE)
    px(img, ox+6, oy+5, RED)
    px(img, ox+9, oy+5, RED)
    # Top of head
    px(img, ox+6, oy+2, ORANGE)
    px(img, ox+7, oy+2, ORANGE)
    px(img, ox+8, oy+2, ORANGE)
    px(img, ox+9, oy+2, ORANGE)
    # Antennae
    px(img, ox+5, oy+1, YELLOW)
    px(img, ox+10, oy+1, YELLOW)
    px(img, ox+4, oy+0, YELLOW)
    px(img, ox+11, oy+0, YELLOW)
    # Small wings out
    px(img, ox+3, oy+5, ORANGE)
    px(img, ox+2, oy+6, ORANGE)
    px(img, ox+3, oy+6, ORANGE)
    px(img, ox+12, oy+5, ORANGE)
    px(img, ox+12, oy+6, ORANGE)
    px(img, ox+13, oy+6, ORANGE)
    # Stinger
    px(img, ox+7, oy+11, ORANGE)
    px(img, ox+8, oy+11, ORANGE)
    px(img, ox+7, oy+12, DARK_ORANGE)
    px(img, ox+8, oy+12, DARK_ORANGE)


def draw_bee_f2(img, ox, oy):
    """Bee/Zako enemy - frame 2 (wings different position)."""
    # Same body
    for x in range(5, 11):
        px(img, ox+x, oy+3, YELLOW)
        px(img, ox+x, oy+10, YELLOW)
    for y in range(4, 10):
        for x in range(4, 12):
            px(img, ox+x, oy+y, YELLOW)
    for x in range(4, 12):
        px(img, ox+x, oy+5, DARK_ORANGE)
        px(img, ox+x, oy+7, DARK_ORANGE)
        px(img, ox+x, oy+9, DARK_ORANGE)
    px(img, ox+5, oy+4, WHITE)
    px(img, ox+6, oy+4, WHITE)
    px(img, ox+9, oy+4, WHITE)
    px(img, ox+10, oy+4, WHITE)
    px(img, ox+6, oy+5, RED)
    px(img, ox+9, oy+5, RED)
    px(img, ox+6, oy+2, ORANGE)
    px(img, ox+7, oy+2, ORANGE)
    px(img, ox+8, oy+2, ORANGE)
    px(img, ox+9, oy+2, ORANGE)
    px(img, ox+5, oy+1, YELLOW)
    px(img, ox+10, oy+1, YELLOW)
    px(img, ox+4, oy+0, YELLOW)
    px(img, ox+11, oy+0, YELLOW)
    # Wings up position
    px(img, ox+3, oy+4, ORANGE)
    px(img, ox+2, oy+3, ORANGE)
    px(img, ox+3, oy+3, ORANGE)
    px(img, ox+2, oy+4, ORANGE)
    px(img, ox+12, oy+4, ORANGE)
    px(img, ox+12, oy+3, ORANGE)
    px(img, ox+13, oy+3, ORANGE)
    px(img, ox+13, oy+4, ORANGE)
    # Stinger
    px(img, ox+7, oy+11, ORANGE)
    px(img, ox+8, oy+11, ORANGE)
    px(img, ox+7, oy+12, DARK_ORANGE)
    px(img, ox+8, oy+12, DARK_ORANGE)


def draw_scorpion_f1(img, ox, oy):
    """Scorpion enemy - frame 1."""
    # Body
    for y in range(4, 12):
        for x in range(5, 11):
            px(img, ox+x, oy+y, RED)
    # Head
    for x in range(6, 10):
        px(img, ox+x, oy+3, RED)
    px(img, ox+7, oy+2, DARK_RED)
    px(img, ox+8, oy+2, DARK_RED)
    # Eyes
    px(img, ox+6, oy+4, YELLOW)
    px(img, ox+9, oy+4, YELLOW)
    # Pincers
    px(img, ox+4, oy+3, RED)
    px(img, ox+3, oy+2, RED)
    px(img, ox+3, oy+3, DARK_RED)
    px(img, ox+11, oy+3, RED)
    px(img, ox+12, oy+2, RED)
    px(img, ox+12, oy+3, DARK_RED)
    # Legs
    for dy in [6, 8, 10]:
        px(img, ox+3, oy+dy, RED)
        px(img, ox+4, oy+dy, RED)
        px(img, ox+2, oy+dy+1, DARK_RED)
        px(img, ox+11, oy+dy, RED)
        px(img, ox+12, oy+dy, RED)
        px(img, ox+13, oy+dy+1, DARK_RED)
    # Tail
    px(img, ox+7, oy+12, RED)
    px(img, ox+8, oy+12, RED)
    px(img, ox+7, oy+13, DARK_RED)
    px(img, ox+8, oy+13, DARK_RED)
    px(img, ox+8, oy+14, RED)


def draw_scorpion_f2(img, ox, oy):
    """Scorpion enemy - frame 2 (legs moved)."""
    for y in range(4, 12):
        for x in range(5, 11):
            px(img, ox+x, oy+y, RED)
    for x in range(6, 10):
        px(img, ox+x, oy+3, RED)
    px(img, ox+7, oy+2, DARK_RED)
    px(img, ox+8, oy+2, DARK_RED)
    px(img, ox+6, oy+4, YELLOW)
    px(img, ox+9, oy+4, YELLOW)
    px(img, ox+4, oy+3, RED)
    px(img, ox+3, oy+2, RED)
    px(img, ox+3, oy+3, DARK_RED)
    px(img, ox+11, oy+3, RED)
    px(img, ox+12, oy+2, RED)
    px(img, ox+12, oy+3, DARK_RED)
    # Legs - different position
    for dy in [5, 7, 9]:
        px(img, ox+3, oy+dy, RED)
        px(img, ox+4, oy+dy, RED)
        px(img, ox+2, oy+dy+1, DARK_RED)
        px(img, ox+11, oy+dy, RED)
        px(img, ox+12, oy+dy, RED)
        px(img, ox+13, oy+dy+1, DARK_RED)
    px(img, ox+7, oy+12, RED)
    px(img, ox+8, oy+12, RED)
    px(img, ox+7, oy+13, DARK_RED)
    px(img, ox+8, oy+13, DARK_RED)
    px(img, ox+8, oy+14, RED)


def draw_stingray_f1(img, ox, oy):
    """Stingray enemy - frame 1."""
    # Wide flat body
    for y in range(5, 10):
        for x in range(3, 13):
            px(img, ox+x, oy+y, CYAN)
    for y in range(6, 9):
        for x in range(2, 14):
            px(img, ox+x, oy+y, CYAN)
    # Darker center
    for y in range(6, 9):
        for x in range(6, 10):
            px(img, ox+x, oy+y, DARK_BLUE)
    # Eyes
    px(img, ox+5, oy+6, WHITE)
    px(img, ox+10, oy+6, WHITE)
    px(img, ox+5, oy+7, YELLOW)
    px(img, ox+10, oy+7, YELLOW)
    # Head point
    px(img, ox+7, oy+3, CYAN)
    px(img, ox+8, oy+3, CYAN)
    px(img, ox+6, oy+4, CYAN)
    px(img, ox+7, oy+4, LIGHT_BLUE)
    px(img, ox+8, oy+4, LIGHT_BLUE)
    px(img, ox+9, oy+4, CYAN)
    # Wing tips
    px(img, ox+1, oy+7, DARK_BLUE)
    px(img, ox+14, oy+7, DARK_BLUE)
    # Tail
    px(img, ox+7, oy+10, CYAN)
    px(img, ox+8, oy+10, CYAN)
    px(img, ox+7, oy+11, DARK_BLUE)
    px(img, ox+8, oy+11, DARK_BLUE)
    px(img, ox+7, oy+12, CYAN)
    px(img, ox+8, oy+12, CYAN)


def draw_stingray_f2(img, ox, oy):
    """Stingray - frame 2 (slightly different wing angle)."""
    for y in range(5, 10):
        for x in range(3, 13):
            px(img, ox+x, oy+y, CYAN)
    for y in range(7, 9):
        for x in range(2, 14):
            px(img, ox+x, oy+y, CYAN)
    for y in range(6, 9):
        for x in range(6, 10):
            px(img, ox+x, oy+y, DARK_BLUE)
    px(img, ox+5, oy+6, WHITE)
    px(img, ox+10, oy+6, WHITE)
    px(img, ox+5, oy+7, YELLOW)
    px(img, ox+10, oy+7, YELLOW)
    px(img, ox+7, oy+3, CYAN)
    px(img, ox+8, oy+3, CYAN)
    px(img, ox+6, oy+4, CYAN)
    px(img, ox+7, oy+4, LIGHT_BLUE)
    px(img, ox+8, oy+4, LIGHT_BLUE)
    px(img, ox+9, oy+4, CYAN)
    px(img, ox+1, oy+8, DARK_BLUE)
    px(img, ox+14, oy+8, DARK_BLUE)
    px(img, ox+7, oy+10, CYAN)
    px(img, ox+8, oy+10, CYAN)
    px(img, ox+7, oy+11, DARK_BLUE)
    px(img, ox+8, oy+11, DARK_BLUE)
    px(img, ox+7, oy+12, CYAN)
    px(img, ox+8, oy+12, CYAN)


def draw_flagship(img, ox, oy):
    """Flagship / boss ship."""
    # Large imposing ship shape
    for y in range(3, 13):
        for x in range(5, 11):
            px(img, ox+x, oy+y, GREEN)
    for x in range(4, 12):
        px(img, ox+x, oy+5, LIGHT_GREEN)
        px(img, ox+x, oy+6, GREEN)
        px(img, ox+x, oy+7, GREEN)
    px(img, ox+7, oy+1, YELLOW)
    px(img, ox+8, oy+1, YELLOW)
    px(img, ox+6, oy+2, GREEN)
    px(img, ox+7, oy+2, YELLOW)
    px(img, ox+8, oy+2, YELLOW)
    px(img, ox+9, oy+2, GREEN)
    # Crown detail
    px(img, ox+5, oy+3, YELLOW)
    px(img, ox+6, oy+3, GREEN)
    px(img, ox+7, oy+3, YELLOW)
    px(img, ox+8, oy+3, YELLOW)
    px(img, ox+9, oy+3, GREEN)
    px(img, ox+10, oy+3, YELLOW)
    # Eyes
    px(img, ox+6, oy+5, RED)
    px(img, ox+9, oy+5, RED)
    # Wing extensions
    for y in range(6, 10):
        px(img, ox+3, oy+y, DARK_GREEN)
        px(img, ox+12, oy+y, DARK_GREEN)
    px(img, ox+2, oy+7, GREEN)
    px(img, ox+2, oy+8, GREEN)
    px(img, ox+13, oy+7, GREEN)
    px(img, ox+13, oy+8, GREEN)
    # Bottom
    px(img, ox+6, oy+12, DARK_GREEN)
    px(img, ox+9, oy+12, DARK_GREEN)
    px(img, ox+7, oy+13, GREEN)
    px(img, ox+8, oy+13, GREEN)


def draw_flag_icon(img, ox, oy, color1, color2, text_val):
    """Draw a small flag icon in 16x16 tile."""
    # Flag pole
    for y in range(2, 14):
        px(img, ox+4, oy+y, WHITE)
    # Flag triangle
    for dy in range(0, 6):
        for dx in range(5, 5 + 8 - dy):
            if ox+dx < ox+16:
                px(img, ox+dx, oy+2+dy, color1)
    # Border
    for dy in range(0, 6):
        x_end = 5 + 8 - dy - 1
        if ox+x_end < ox+16:
            px(img, ox+x_end, oy+2+dy, color2)
    # Base
    px(img, ox+3, oy+13, GRAY)
    px(img, ox+4, oy+13, GRAY)
    px(img, ox+5, oy+13, GRAY)


def draw_explosion(img, ox, oy, frame):
    """Draw explosion frame (0-5), expanding burst."""
    center_x, center_y = 7, 7
    if frame == 0:
        # Small bright core
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                px(img, ox+center_x+dx, oy+center_y+dy, WHITE)
    elif frame == 1:
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                px(img, ox+center_x+dx, oy+center_y+dy, WHITE)
        for dx in range(-2, 3):
            px(img, ox+center_x+dx, oy+center_y, YELLOW)
        for dy in range(-2, 3):
            px(img, ox+center_x, oy+center_y+dy, YELLOW)
    elif frame == 2:
        for dx in range(-2, 3):
            for dy in range(-2, 3):
                px(img, ox+center_x+dx, oy+center_y+dy, YELLOW)
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                px(img, ox+center_x+dx, oy+center_y+dy, WHITE)
        # Rays
        for d in range(-3, 4):
            px(img, ox+center_x+d, oy+center_y, ORANGE)
            px(img, ox+center_x, oy+center_y+d, ORANGE)
    elif frame == 3:
        # Larger explosion
        for dx in range(-3, 4):
            for dy in range(-3, 4):
                if abs(dx) + abs(dy) <= 4:
                    px(img, ox+center_x+dx, oy+center_y+dy, ORANGE)
        for dx in range(-2, 3):
            for dy in range(-2, 3):
                px(img, ox+center_x+dx, oy+center_y+dy, YELLOW)
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                px(img, ox+center_x+dx, oy+center_y+dy, WHITE)
        # Sparks
        for d in range(-5, 6):
            if abs(d) > 3:
                px(img, ox+center_x+d, oy+center_y, YELLOW)
                px(img, ox+center_x, oy+center_y+d, YELLOW)
    elif frame == 4:
        # Breaking apart
        for dx in range(-4, 5):
            for dy in range(-4, 5):
                if abs(dx) + abs(dy) <= 5 and abs(dx) + abs(dy) > 1:
                    c = ORANGE if (dx + dy) % 2 == 0 else YELLOW
                    px(img, ox+center_x+dx, oy+center_y+dy, c)
        # Sparks flying out
        for d in [-6, -5, 5, 6]:
            px(img, ox+center_x+d, oy+center_y, RED)
            px(img, ox+center_x, oy+center_y+d, RED)
        for d in [-4, 4]:
            px(img, ox+center_x+d, oy+center_y+d, ORANGE)
            px(img, ox+center_x+d, oy+center_y-d, ORANGE)
    elif frame == 5:
        # Fading debris
        import random
        random.seed(42)
        for _ in range(20):
            dx = random.randint(-6, 6)
            dy = random.randint(-6, 6)
            if abs(dx) + abs(dy) > 2:
                c = [RED, ORANGE, DARK_ORANGE, YELLOW][random.randint(0, 3)]
                px(img, ox+center_x+dx, oy+center_y+dy, c)


def draw_beam_frame(img, ox, oy, frame):
    """Draw tractor beam frame 0-3 in the enemy spritesheet."""
    # Narrow beam expanding downward
    width = 2 + frame * 2
    for y in range(0, 16):
        w = max(1, int(width * y / 16))
        for dx in range(-w, w + 1):
            alpha = max(60, 255 - abs(dx) * 40 - frame * 20)
            c = (0, 200, 255, min(255, alpha))
            px(img, ox + 8 + dx, oy + y, c)
    # Bright center line
    for y in range(0, 16):
        px(img, ox + 8, oy + y, CYAN)
        if frame > 0:
            px(img, ox + 7, oy + y, LIGHT_BLUE)
            px(img, ox + 9, oy + y, LIGHT_BLUE)


def draw_text_number(img, ox, oy, num_str, color):
    """Draw a tiny number string using 3x5 font in a 16x16 tile."""
    # Very simple 3x5 digit bitmaps
    digits = {
        '1': [
            (1,0),(0,1),(1,1),(1,2),(1,3),(0,4),(1,4),(2,4),
        ],
        '5': [
            (0,0),(1,0),(2,0),(0,1),(0,2),(1,2),(2,2),(2,3),(0,4),(1,4),(2,4),
        ],
        '1_full': [
            (1,0),(0,1),(1,1),(1,2),(1,3),(0,4),(1,4),(2,4),
        ],
        '0': [
            (0,0),(1,0),(2,0),(0,1),(2,1),(0,2),(2,2),(0,3),(2,3),(0,4),(1,4),(2,4),
        ],
        '2': [
            (0,0),(1,0),(2,0),(2,1),(0,2),(1,2),(2,2),(0,3),(0,4),(1,4),(2,4),
        ],
        '3': [
            (0,0),(1,0),(2,0),(2,1),(0,2),(1,2),(2,2),(2,3),(0,4),(1,4),(2,4),
        ],
    }
    cx = ox + 5
    cy = oy + 6
    for i, ch in enumerate(num_str):
        if ch in digits:
            for dx, dy in digits[ch]:
                px(img, cx + dx + i * 4, cy + dy, color)


def make_enemies():
    img = Image.new("RGBA", (128, 64), TRANSPARENT)

    # Row 0 (y=0): Boss green f1, Boss green f2, Boss purple f1, Boss purple f2, Butterfly f1, Butterfly f2, Bee f1, Bee f2
    draw_boss_green_f1(img, 0, 0)
    draw_boss_green_f2(img, 16, 0)
    draw_boss_purple_f1(img, 32, 0)
    draw_boss_purple_f2(img, 48, 0)
    draw_butterfly_f1(img, 64, 0)
    draw_butterfly_f2(img, 80, 0)
    draw_bee_f1(img, 96, 0)
    draw_bee_f2(img, 112, 0)

    # Row 1 (y=16): Scorpion f1, Scorpion f2, Stingray f1, Stingray f2, Flagship, Flag-1, Flag-5, Flag-10
    draw_scorpion_f1(img, 0, 16)
    draw_scorpion_f2(img, 16, 16)
    draw_stingray_f1(img, 32, 16)
    draw_stingray_f2(img, 48, 16)
    draw_flagship(img, 64, 16)
    draw_flag_icon(img, 80, 16, RED, DARK_RED, "1")
    draw_flag_icon(img, 96, 16, BLUE, DARK_BLUE, "5")
    draw_text_number(img, 80, 16, "1", WHITE)
    draw_text_number(img, 96, 16, "5", WHITE)
    # Flag-10
    draw_flag_icon(img, 112, 16, GREEN, DARK_GREEN, "10")
    draw_text_number(img, 112, 16, "10", WHITE)

    # Row 2 (y=32): Explosion frames 1-6, Beam frames 1-2
    for i in range(6):
        draw_explosion(img, i * 16, 32, i)
    draw_beam_frame(img, 96, 32, 0)
    draw_beam_frame(img, 112, 32, 1)

    # Row 3 (y=48): Beam frames 3-4, Flag-20, Flag-30, Flag-50, (empty x3)
    draw_beam_frame(img, 0, 48, 2)
    draw_beam_frame(img, 16, 48, 3)
    draw_flag_icon(img, 32, 48, YELLOW, DARK_ORANGE, "20")
    draw_text_number(img, 32, 48, "20", WHITE)
    draw_flag_icon(img, 48, 48, PURPLE, DARK_PURPLE, "30")
    draw_text_number(img, 48, 48, "30", WHITE)
    draw_flag_icon(img, 64, 48, CYAN, DARK_BLUE, "50")
    draw_text_number(img, 64, 48, "50", WHITE)

    img.save(os.path.join(OUT, "enemies.png"))
    print("  enemies.png")


# =============================================================================
# 3. BULLETS (4x16)
# =============================================================================
def make_bullets():
    img = Image.new("RGBA", (4, 16), TRANSPARENT)
    # Top 8 rows: player bullet (white/yellow thin line)
    for y in range(1, 7):
        px(img, 1, y, WHITE)
        px(img, 2, y, WHITE)
    px(img, 1, 0, YELLOW)
    px(img, 2, 0, YELLOW)
    px(img, 1, 7, YELLOW)
    px(img, 2, 7, YELLOW)

    # Bottom 8 rows: enemy bullet (red)
    for y in range(9, 15):
        px(img, 1, y, RED)
        px(img, 2, y, RED)
    px(img, 1, 8, YELLOW)
    px(img, 2, 8, YELLOW)
    px(img, 1, 15, DARK_RED)
    px(img, 2, 15, DARK_RED)

    img.save(os.path.join(OUT, "bullets.png"))
    print("  bullets.png")


# =============================================================================
# 4. TRACTOR BEAM (16x64, 4 frames of 16x16 stacked)
# =============================================================================
def make_tractor_beam():
    img = Image.new("RGBA", (16, 64), TRANSPARENT)
    for frame in range(4):
        oy = frame * 16
        # Each frame: cone expanding downward
        spread = 1.0 + frame * 0.5  # Increasing spread
        for y in range(16):
            half_w = max(1, int((y / 15.0) * 7 * spread))
            half_w = min(half_w, 7)
            for dx in range(-half_w, half_w + 1):
                x = 8 + dx
                if 0 <= x < 16:
                    dist = abs(dx) / max(half_w, 1)
                    if dist < 0.3:
                        c = CYAN
                    elif dist < 0.6:
                        c = LIGHT_BLUE
                    elif dist < 0.85:
                        c = BLUE
                    else:
                        c = (0, 50, 180, 150)
                    # Flickering effect per frame
                    if frame % 2 == 1 and y % 3 == 0 and abs(dx) > 1:
                        c = CYAN if c == BLUE else c
                    px(img, x, oy + y, c)
        # Bright center line
        for y in range(16):
            px(img, 7, oy + y, WHITE)
            px(img, 8, oy + y, WHITE)

    img.save(os.path.join(OUT, "tractor_beam.png"))
    print("  tractor_beam.png")


# =============================================================================
# 5. STAGE FLAGS (48x8, 6 small 8x8 icons)
# =============================================================================
def make_stage_flags():
    img = Image.new("RGBA", (48, 8), TRANSPARENT)

    flag_colors = [
        (RED, DARK_RED),       # 1
        (BLUE, DARK_BLUE),     # 5
        (GREEN, DARK_GREEN),   # 10
        (YELLOW, DARK_ORANGE), # 20
        (PURPLE, DARK_PURPLE), # 30
        (CYAN, DARK_BLUE),     # 50
    ]

    for i, (c1, c2) in enumerate(flag_colors):
        ox = i * 8
        # Pole
        for y in range(1, 7):
            px(img, ox + 1, y, WHITE)
        # Flag triangle (small)
        for dy in range(0, 4):
            for dx in range(2, 2 + 5 - dy):
                if ox + dx < ox + 8:
                    px(img, ox + dx, 1 + dy, c1)
        # Border edge
        for dy in range(0, 4):
            x_end = 2 + 5 - dy - 1
            if ox + x_end < ox + 8:
                px(img, ox + x_end, 1 + dy, c2)
        # Base
        px(img, ox + 0, 7, GRAY)
        px(img, ox + 1, 7, GRAY)
        px(img, ox + 2, 7, GRAY)

    img.save(os.path.join(OUT, "stage_flags.png"))
    print("  stage_flags.png")


# =============================================================================
# Main
# =============================================================================
if __name__ == "__main__":
    print("Generating Galaga sprites...")
    make_player()
    make_enemies()
    make_bullets()
    make_tractor_beam()
    make_stage_flags()
    print("Done! Sprites saved to:", OUT)
