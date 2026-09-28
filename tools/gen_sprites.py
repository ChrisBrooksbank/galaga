"""
Generate sprite assets for the Galaga clone.

Sprite atlas layout (enemies.png, 128x64 = 8 cols x 4 rows of 16x16):
  Row 0: Boss G1, Boss G2, Boss P1, Boss P2, Butterfly1, Butterfly2, Bee1, Bee2
  Row 1: Scorpion1, Scorpion2, Stingray1, Stingray2, Flagship, Flag1, Flag5, Flag10
  Row 2: Expl1, Expl2, Expl3, Expl4, Expl5, Expl6, Beam1, Beam2
  Row 3: Beam3, Beam4, Flag20, Flag30, Flag50, (empty x3)
"""

import struct
import zlib
import os

TRANSPARENT = (0, 0, 0, 0)


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


def make_canvas(width, height, fill=TRANSPARENT):
    return [fill] * (width * height)


def blit(pixels, atlas_width, x0, y0, tile, tile_w=16, tile_h=16):
    for y in range(tile_h):
        for x in range(tile_w):
            c = tile[y * tile_w + x]
            if c[3] != 0:
                pixels[(y0 + y) * atlas_width + (x0 + x)] = c


def shade(color, factor):
    """Darken (factor<1) or lighten (factor>1) an RGBA color, clamped to 0..255."""
    r, g, b, a = color
    return (
        max(0, min(255, int(r * factor))),
        max(0, min(255, int(g * factor))),
        max(0, min(255, int(b * factor))),
        a,
    )


# ── Generic insect-silhouette renderer ──────────────────────────────────────
#
# Galaga's Bee/Butterfly/Boss enemies share a recognizable "bowtie" silhouette:
# wide swept wings top and bottom, a narrow thorax waist in the middle, and a
# small head/antennae nub at the very top. `wing_profile` gives the half-width
# (from the vertical centerline) of the wing silhouette per row; `body_profile`
# gives the half-width of the narrower thorax stripe drawn in a second color on
# top of it. `flutter` nudges the wing rows to animate a flap between frames.

def render_insect(wing_profile, body_profile, wing_color, body_color,
                   outline_color, eye_color=None, flutter=0, antenna_color=None):
    tile = make_canvas(16, 16)
    profile = list(wing_profile)
    if flutter:
        # Flap: widen the upper wing rows and narrow the lower wing rows (or
        # vice versa) so alternating frames look like a wingbeat.
        for i in (3, 4):
            if i < len(profile) and profile[i] > 0:
                profile[i] = max(0, profile[i] + flutter)
        for i in (10, 11):
            if i < len(profile) and profile[i] > 0:
                profile[i] = max(0, profile[i] - flutter)

    for y in range(16):
        hw = profile[y]
        if hw <= 0:
            continue
        bw = body_profile[y] if y < len(body_profile) else 0
        for x in range(16):
            dx = abs(x - 7.5)
            if dx <= hw:
                if bw and dx <= bw:
                    tile[y * 16 + x] = body_color
                else:
                    tile[y * 16 + x] = wing_color

    # Antennae: two single pixels poking above the topmost filled row.
    if antenna_color:
        top = next((y for y, hw in enumerate(profile) if hw > 0), None)
        if top is not None and top >= 1:
            for x in (6, 9):
                tile[(top - 1) * 16 + x] = antenna_color

    # 1px outline: any transparent pixel adjacent to a filled one.
    filled = lambda x, y: 0 <= x < 16 and 0 <= y < 16 and tile[y * 16 + x][3] != 0
    outline_px = []
    for y in range(16):
        for x in range(16):
            if filled(x, y):
                continue
            if filled(x - 1, y) or filled(x + 1, y) or filled(x, y - 1) or filled(x, y + 1):
                outline_px.append((x, y))
    for x, y in outline_px:
        tile[y * 16 + x] = outline_color

    # Eye/core accent: small 2x2 dot at the thorax center.
    if eye_color:
        cy = 7
        for y in (cy, cy + 1):
            for x in (7, 8):
                if tile[y * 16 + x][3] != 0:
                    tile[y * 16 + x] = eye_color

    return tile


# Shared bowtie profile used by Bee/Butterfly/Boss (wide wings, narrow waist).
INSECT_WINGS = [1, 2, 3, 5, 6, 6, 4, 3, 3, 4, 6, 6, 5, 3, 2, 1]
INSECT_BODY = [0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 0, 0]

# Boss Galaga: broader and taller, with a pronounced double-horn head.
BOSS_WINGS = [0, 2, 3, 5, 7, 7, 5, 4, 4, 5, 7, 7, 6, 4, 3, 1]
BOSS_BODY = [0, 0, 1, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 1, 1, 0]

# Scorpion (splitter): narrower body, spiked tail curling at the bottom.
SCORPION_WINGS = [0, 1, 2, 4, 5, 5, 3, 2, 2, 3, 4, 3, 2, 2, 1, 1]
SCORPION_BODY = [0, 0, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1, 1, 0, 0, 0]

# Stingray (splitter): flat wide "ray" body with a tapering tail.
STINGRAY_WINGS = [0, 0, 2, 4, 6, 7, 6, 4, 3, 2, 2, 1, 1, 1, 0, 0]
STINGRAY_BODY = [0, 0, 0, 1, 2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0]

# Galaxian-style flagship: flat wide chevron, distinct from the round insects.
FLAGSHIP_WINGS = [0, 0, 1, 3, 5, 7, 7, 6, 6, 7, 7, 5, 3, 1, 0, 0]
FLAGSHIP_BODY = [0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 0, 0, 0]


def gen_enemies_atlas():
    W, H = 128, 64
    pixels = make_canvas(W, H)

    def put(col, row, tile):
        blit(pixels, W, col * 16, row * 16, tile)

    # ---- Row 0: Boss (green + damaged purple), Butterfly, Bee ----
    for i, flutter in enumerate((1, -1)):
        put(0 + i, 0, render_insect(
            BOSS_WINGS, BOSS_BODY,
            wing_color=(20, 150, 130, 255), body_color=(120, 230, 90, 255),
            outline_color=(5, 60, 55, 255), eye_color=(255, 60, 180, 255),
            flutter=flutter, antenna_color=(230, 230, 230, 255),
        ))
    for i, flutter in enumerate((1, -1)):
        put(2 + i, 0, render_insect(
            BOSS_WINGS, BOSS_BODY,
            wing_color=(150, 20, 140, 255), body_color=(230, 120, 210, 255),
            outline_color=(60, 5, 55, 255), eye_color=(255, 230, 60, 255),
            flutter=flutter, antenna_color=(230, 230, 230, 255),
        ))
    for i, flutter in enumerate((1, -1)):
        put(4 + i, 0, render_insect(
            INSECT_WINGS, INSECT_BODY,
            wing_color=(210, 20, 30, 255), body_color=(250, 250, 250, 255),
            outline_color=(90, 5, 10, 255), eye_color=(255, 220, 40, 255),
            flutter=flutter, antenna_color=(210, 20, 30, 255),
        ))
    for i, flutter in enumerate((1, -1)):
        put(6 + i, 0, render_insect(
            INSECT_WINGS, INSECT_BODY,
            wing_color=(30, 100, 220, 255), body_color=(255, 210, 30, 255),
            outline_color=(10, 35, 90, 255), eye_color=(255, 255, 255, 255),
            flutter=flutter, antenna_color=(30, 100, 220, 255),
        ))

    # ---- Row 1: Scorpion, Stingray, Flagship, stage flags 1/5/10 ----
    for i, flutter in enumerate((1, -1)):
        put(0 + i, 1, render_insect(
            SCORPION_WINGS, SCORPION_BODY,
            wing_color=(210, 190, 20, 255), body_color=(255, 240, 140, 255),
            outline_color=(90, 75, 5, 255), eye_color=(200, 20, 20, 255),
            flutter=flutter,
        ))
    for i, flutter in enumerate((1, -1)):
        put(2 + i, 1, render_insect(
            STINGRAY_WINGS, STINGRAY_BODY,
            wing_color=(20, 170, 110, 255), body_color=(160, 255, 210, 255),
            outline_color=(5, 70, 45, 255), eye_color=(255, 255, 255, 255),
            flutter=flutter,
        ))
    put(4, 1, render_insect(
        FLAGSHIP_WINGS, FLAGSHIP_BODY,
        wing_color=(220, 110, 10, 255), body_color=(255, 210, 120, 255),
        outline_color=(100, 45, 0, 255), eye_color=(255, 60, 60, 255),
    ))
    put(5, 1, make_flag_tile((235, 235, 235, 255)))   # stage flag: 1
    put(6, 1, make_flag_tile((90, 200, 240, 255)))    # stage flag: 5
    put(7, 1, make_flag_tile((240, 90, 90, 255)))     # stage flag: 10

    # ---- Row 2: Explosion frames 1-6, tractor beam frames 1-2 ----
    for i in range(6):
        put(i, 2, make_explosion_tile(i))
    put(6, 2, make_beam_tile(0))
    put(7, 2, make_beam_tile(1))

    # ---- Row 3: tractor beam frames 3-4, stage flags 20/30/50 ----
    put(0, 3, make_beam_tile(2))
    put(1, 3, make_beam_tile(3))
    put(2, 3, make_flag_tile((240, 200, 60, 255)))    # stage flag: 20
    put(3, 3, make_flag_tile((150, 90, 230, 255)))    # stage flag: 30
    put(4, 3, make_flag_tile((230, 60, 150, 255)))    # stage flag: 50
    # Slots 5-7 remain empty (transparent)

    save_png("assets/sprites/enemies.png", W, H, pixels)


def make_explosion_tile(frame):
    """A radiating burst (diamond ring) that expands then breaks into embers."""
    stages = [
        (3, (255, 255, 220, 255), (255, 230, 120, 255)),
        (5, (255, 220, 110, 255), (255, 170, 40, 255)),
        (7, (255, 170, 60, 255), (230, 110, 20, 255)),
        (7, (230, 110, 30, 255), (170, 60, 10, 255)),
        (6, (170, 70, 20, 255), (110, 35, 5, 255)),
        (4, (110, 45, 15, 255), (60, 20, 5, 255)),
    ]
    radius, core, edge = stages[min(frame, 5)]
    tile = make_canvas(16, 16)
    cx = cy = 7.5
    for y in range(16):
        for x in range(16):
            d = abs(x - cx) + abs(y - cy)  # diamond (Manhattan) burst
            if d <= radius:
                if frame >= 4 and (x + y) % 3 == 0:
                    continue  # embers: later frames get gappy/fragmented
                tile[y * 16 + x] = core if d <= radius - 2 else edge
    return tile


def make_beam_tile(frame):
    """Fan-shaped tractor beam with a scalloped, faintly striped edge."""
    tile = make_canvas(16, 16)
    alpha = 210 - frame * 25
    for y in range(16):
        fan = (frame + 1) * 1.6 + y * 0.25
        cx = 7.5
        for x in range(16):
            dx = abs(x - cx)
            if dx <= fan:
                # Scalloped brightness: alternating bands read as beam "ribs".
                band = int(dx) % 3 == 0
                base = 235 if band else 170
                brightness = max(70, base - int(dx * 10))
                tile[y * 16 + x] = (0, brightness, brightness, alpha)
    return tile


def make_flag_tile(color):
    """A small pennant on a pole, matching a single 16x16 atlas cell."""
    tile = make_canvas(16, 16)
    pole = (150, 130, 90, 255)
    dark = shade(color, 0.55)
    # Pole: vertical bar near the left edge.
    for y in range(2, 15):
        tile[y * 16 + 3] = pole
    # Pennant: right-pointing triangle attached to the pole.
    for row, half in enumerate([5, 4, 3, 2, 1]):
        y = 3 + row
        for x in range(4, 4 + (5 - row) * 2):
            tile[y * 16 + x] = color if x < 12 else dark
    # Outline the pennant's top/bottom edge for definition.
    tile[3 * 16 + 4] = dark
    tile[7 * 16 + 4] = dark
    return tile


# ── Player fighter ───────────────────────────────────────────────────────────

def gen_player():
    W, H = 16, 16
    tile = make_canvas(W, H)
    white = (235, 235, 240, 255)
    red = (220, 30, 30, 255)
    blue = (40, 140, 230, 255)
    dark = (60, 60, 70, 255)

    # Silhouette half-width per row: narrow nose -> flared twin-pod wings.
    profile = [1, 1, 2, 2, 3, 4, 4, 5, 6, 7, 8, 8, 7, 6, 0, 0]
    for y, hw in enumerate(profile):
        for x in range(W):
            if abs(x - 7.5) <= hw:
                tile[y * W + x] = white
    # Engine notch: split the bottom of the hull into twin pods.
    for y in (12, 13):
        for x in range(6, 10):
            tile[y * W + x] = TRANSPARENT
    for x, y in [(4, 14), (5, 14), (10, 14), (11, 14)]:
        tile[y * W + x] = white

    # Red wingtips on the outermost columns of the flared rows.
    for y in (8, 9, 10, 11):
        hw = profile[y]
        for dx in (0, 1):
            for sign in (-1, 1):
                x = int(7.5 + sign * (hw - dx))
                if 0 <= x < W and tile[y * W + x][3] != 0:
                    tile[y * W + x] = red

    # Blue cockpit window.
    for x, y in [(7, 4), (8, 4), (7, 5), (8, 5)]:
        tile[y * W + x] = blue

    # 1px dark outline around the silhouette.
    filled = lambda x, y: 0 <= x < W and 0 <= y < H and tile[y * W + x][3] != 0
    outline_px = []
    for y in range(H):
        for x in range(W):
            if filled(x, y):
                continue
            if filled(x - 1, y) or filled(x + 1, y) or filled(x, y - 1) or filled(x, y + 1):
                outline_px.append((x, y))
    for x, y in outline_px:
        tile[y * W + x] = dark

    save_png("assets/sprites/player.png", W, H, tile)
    return tile


def gen_lives_icon(player_tile):
    """Mini player ship icon for the lives HUD: the player sprite, scaled down."""
    W, H = 16, 16
    tile = make_canvas(W, H)
    for y in range(H):
        for x in range(W):
            sx, sy = x, min(H - 1, y + 1)  # nudge down slightly to re-center
            src = player_tile[sy * W + sx]
            if src[3] != 0:
                tile[y * W + x] = src
    save_png("assets/sprites/ui/lives_icon.png", W, H, tile)


# ── Bullets ──────────────────────────────────────────────────────────────────

def gen_bullets():
    """Player bullet (cyan/white tapered bolt) over enemy bullet (red-orange), 4x16."""
    W, H = 4, 16
    pixels = make_canvas(W, H)
    white = (240, 250, 255, 255)
    cyan = (60, 220, 255, 255)
    orange = (255, 90, 30, 255)
    dark_orange = (170, 40, 10, 255)

    # Player bullet: tapered bolt, bright core with cyan glow, rows 0-7.
    for y in range(1, 7):
        pixels[y * W + 1] = cyan
        pixels[y * W + 2] = cyan
    pixels[1 * W + 1] = TRANSPARENT
    pixels[1 * W + 2] = white
    for y in range(2, 6):
        pixels[y * W + 1] = white if y in (3, 4) else cyan
        pixels[y * W + 2] = white if y in (3, 4) else cyan

    # Enemy bullet: small glowing ember, rows 9-14.
    for y in range(9, 15):
        pixels[y * W + 1] = orange
        pixels[y * W + 2] = orange
    pixels[9 * W + 1] = dark_orange
    pixels[9 * W + 2] = dark_orange
    pixels[14 * W + 1] = dark_orange
    pixels[14 * W + 2] = dark_orange

    save_png("assets/sprites/bullets.png", W, H, pixels)


def gen_tractor_beam():
    """4 frames of the boss's fan-shaped capture beam, matching the atlas beam tiles."""
    W, H = 16, 64
    pixels = make_canvas(W, H)
    for frame in range(4):
        blit(pixels, W, 0, frame * 16, make_beam_tile(frame))
    save_png("assets/sprites/tractor_beam.png", W, H, pixels)


def gen_stage_flags():
    """Stage flag strip: 1, 5, 10, 20, 30, 50 — same art as the atlas flag tiles, 8x8 each."""
    colors = [
        (235, 235, 235, 255),  # 1
        (90, 200, 240, 255),   # 5
        (240, 90, 90, 255),    # 10
        (240, 200, 60, 255),   # 20
        (150, 90, 230, 255),   # 30
        (230, 60, 150, 255),   # 50
    ]
    W, H = 8 * len(colors), 8
    pixels = make_canvas(W, H)
    pole = (150, 130, 90, 255)
    for i, color in enumerate(colors):
        x0 = i * 8
        dark = shade(color, 0.55)
        for y in range(1, 7):
            pixels[y * W + (x0 + 1)] = pole
        for row, half in enumerate([2, 1]):
            y = 1 + row
            for x in range(2, 2 + (3 - row) * 2):
                pixels[y * W + (x0 + x)] = color if x < 6 else dark
    save_png("assets/sprites/stage_flags.png", W, H, pixels)


if __name__ == "__main__":
    print("Generating sprite assets...")
    gen_enemies_atlas()
    player_tile = gen_player()
    gen_lives_icon(player_tile)
    gen_bullets()
    gen_tractor_beam()
    gen_stage_flags()
    print("Done.")
