"""
Generate retro arcade sound effects and music for Galaga.

Sound effects: WAV (mono, 16-bit, 44100 Hz)
Music: WAV -> OGG via ffmpeg
"""

import wave
import struct
import math
import random
import os
import subprocess
import sys

SAMPLE_RATE = 44100
SOUNDS_DIR = os.path.join(os.path.dirname(__file__), "assets", "sounds")
MUSIC_DIR = os.path.join(os.path.dirname(__file__), "assets", "music")


def ensure_dirs():
    os.makedirs(SOUNDS_DIR, exist_ok=True)
    os.makedirs(MUSIC_DIR, exist_ok=True)


# --------------- wave generators ---------------

def sine(freq, t, phase=0.0):
    return math.sin(2.0 * math.pi * freq * t + phase)


def square(freq, t, duty=0.5):
    cycle = (t * freq) % 1.0
    return 1.0 if cycle < duty else -1.0


def triangle(freq, t):
    cycle = (t * freq) % 1.0
    if cycle < 0.25:
        return 4.0 * cycle
    elif cycle < 0.75:
        return 2.0 - 4.0 * cycle
    else:
        return -4.0 + 4.0 * cycle


def sawtooth(freq, t):
    cycle = (t * freq) % 1.0
    return 2.0 * cycle - 1.0


def noise():
    return random.uniform(-1.0, 1.0)


def envelope_adsr(t, duration, attack=0.01, decay=0.05, sustain_level=0.7, release=0.05):
    """Simple ADSR envelope."""
    release_start = duration - release
    if release_start < 0:
        release_start = 0
    if t < attack:
        return t / attack if attack > 0 else 1.0
    elif t < attack + decay:
        return 1.0 - (1.0 - sustain_level) * ((t - attack) / decay) if decay > 0 else sustain_level
    elif t < release_start:
        return sustain_level
    elif t < duration:
        return sustain_level * (1.0 - (t - release_start) / release) if release > 0 else 0.0
    return 0.0


def envelope_linear(t, duration, fade_in=0.005, fade_out=0.01):
    """Simple linear fade in/out."""
    if t < fade_in:
        return t / fade_in if fade_in > 0 else 1.0
    elif t > duration - fade_out:
        remaining = duration - t
        return remaining / fade_out if fade_out > 0 else 0.0
    return 1.0


def write_wav(filepath, samples, sample_rate=SAMPLE_RATE):
    """Write mono 16-bit WAV."""
    with wave.open(filepath, 'w') as wf:
        wf.setnchannels(1)
        wf.setsampwidth(2)
        wf.setframerate(sample_rate)
        data = b''
        for s in samples:
            s = max(-1.0, min(1.0, s))
            data += struct.pack('<h', int(s * 32767))
        wf.writeframes(data)


def generate_samples(duration, generator_func):
    """Generate samples for a given duration using a generator function.
    generator_func(t, duration) -> float in [-1, 1]
    """
    n = int(SAMPLE_RATE * duration)
    return [generator_func(i / SAMPLE_RATE, duration) for i in range(n)]


# --------------- Sound Effects ---------------

def gen_shoot():
    """Short laser/pew - descending high pitch blip."""
    dur = 0.12
    def gen(t, d):
        freq = 1800 - 1200 * (t / d)  # descend from 1800 to 600
        env = envelope_linear(t, d, 0.002, 0.03)
        return (0.6 * square(freq, t, 0.25) + 0.4 * sine(freq * 2, t)) * env * 0.8
    return generate_samples(dur, gen)


def gen_enemy_shoot():
    """Enemy shot - slightly lower, different character."""
    dur = 0.15
    def gen(t, d):
        freq = 1200 - 800 * (t / d)
        env = envelope_linear(t, d, 0.002, 0.04)
        return (0.5 * square(freq, t, 0.35) + 0.3 * sawtooth(freq * 0.5, t) + 0.2 * sine(freq, t)) * env * 0.7
    return generate_samples(dur, gen)


def gen_explosion_small():
    """Short explosion - noise burst with pitch."""
    dur = 0.25
    def gen(t, d):
        env = math.exp(-6.0 * t / d)
        freq = 200 - 150 * (t / d)
        return (0.7 * noise() + 0.3 * square(freq, t, 0.5)) * env * 0.85
    return generate_samples(dur, gen)


def gen_explosion_large():
    """Longer deeper explosion."""
    dur = 0.6
    def gen(t, d):
        env = math.exp(-4.0 * t / d)
        freq = 120 - 80 * (t / d)
        n = noise()
        # Low-pass approximation: mix more low-freq content
        return (0.5 * n + 0.3 * square(freq, t) + 0.2 * sine(freq * 0.5, t)) * env * 0.9
    return generate_samples(dur, gen)


def gen_player_death():
    """Dramatic death - descending tone + noise burst."""
    dur = 1.0
    def gen(t, d):
        # Descending tone
        freq = 800 * math.exp(-3.0 * t / d)
        tone = 0.5 * square(freq, t, 0.4) + 0.3 * sine(freq, t)
        # Noise burst that fades in slightly then out
        noise_env = math.sin(math.pi * t / d) * math.exp(-2.0 * t / d)
        n = noise() * noise_env
        env = envelope_linear(t, d, 0.005, 0.1)
        return (0.6 * tone + 0.4 * n) * env * 0.85
    return generate_samples(dur, gen)


def gen_tractor_beam():
    """Sustained eerie beam sound ~2 sec with warble."""
    dur = 2.0
    def gen(t, d):
        # Wobbling frequency
        wobble = 5.0 + 3.0 * sine(0.8, t)
        freq = 300 + 100 * sine(wobble, t)
        env = envelope_adsr(t, d, 0.1, 0.1, 0.8, 0.3)
        return (0.4 * sine(freq, t) + 0.3 * triangle(freq * 1.5, t) +
                0.2 * sine(freq * 2.01, t) + 0.1 * square(freq * 0.5, t, 0.3)) * env * 0.7
    return generate_samples(dur, gen)


def gen_capture():
    """Capture confirmation - descending ominous tone."""
    dur = 0.5
    def gen(t, d):
        freq = 600 - 300 * (t / d)
        env = envelope_adsr(t, d, 0.01, 0.05, 0.8, 0.1)
        # Add a pulsing effect
        pulse = 0.7 + 0.3 * sine(12, t)
        return (0.5 * square(freq, t, 0.4) + 0.5 * sine(freq * 0.5, t)) * env * pulse * 0.75
    return generate_samples(dur, gen)


def gen_rescue():
    """Happy ascending tones."""
    dur = 0.6
    notes = [523.25, 659.25, 783.99, 1046.50]  # C5, E5, G5, C6
    note_dur = dur / len(notes)
    def gen(t, d):
        idx = min(int(t / note_dur), len(notes) - 1)
        freq = notes[idx]
        local_t = t - idx * note_dur
        env = envelope_adsr(local_t, note_dur, 0.005, 0.02, 0.7, 0.02)
        return (0.6 * square(freq, t, 0.25) + 0.4 * sine(freq, t)) * env * 0.75
    return generate_samples(dur, gen)


def gen_dual_join():
    """Power-up merge - rising sweep with harmonics."""
    dur = 0.5
    def gen(t, d):
        freq = 300 + 900 * (t / d)
        env = envelope_adsr(t, d, 0.02, 0.05, 0.8, 0.05)
        return (0.4 * square(freq, t, 0.25) + 0.3 * sine(freq, t) +
                0.2 * sine(freq * 2, t) + 0.1 * triangle(freq * 3, t)) * env * 0.7
    return generate_samples(dur, gen)


def gen_dive_swoosh():
    """Quick swoosh - fast frequency sweep."""
    dur = 0.2
    def gen(t, d):
        freq = 2000 * math.exp(-8.0 * t / d) + 100
        env = envelope_linear(t, d, 0.005, 0.05)
        return (0.5 * sine(freq, t) + 0.3 * noise() * (1 - t / d) + 0.2 * triangle(freq * 0.5, t)) * env * 0.6
    return generate_samples(dur, gen)


def gen_bonus():
    """Bonus point jingle - quick happy notes."""
    dur = 0.4
    notes = [784, 988, 1175, 1318]  # G5, B5, D6, E6
    note_dur = dur / len(notes)
    def gen(t, d):
        idx = min(int(t / note_dur), len(notes) - 1)
        freq = notes[idx]
        local_t = t - idx * note_dur
        env = envelope_adsr(local_t, note_dur, 0.003, 0.01, 0.7, 0.01)
        return (0.5 * square(freq, t, 0.25) + 0.5 * triangle(freq, t)) * env * 0.7
    return generate_samples(dur, gen)


def gen_extra_life():
    """Extra life - ascending arpeggio, bright and celebratory."""
    dur = 0.8
    # C major arpeggio going up two octaves
    notes = [523.25, 659.25, 783.99, 1046.50, 1318.51, 1567.98]
    note_dur = dur / len(notes)
    def gen(t, d):
        idx = min(int(t / note_dur), len(notes) - 1)
        freq = notes[idx]
        local_t = t - idx * note_dur
        env = envelope_adsr(local_t, note_dur, 0.005, 0.02, 0.8, 0.02)
        overall = envelope_linear(t, d, 0.005, 0.05)
        return (0.4 * square(freq, t, 0.25) + 0.3 * sine(freq, t) +
                0.2 * sine(freq * 2, t) + 0.1 * triangle(freq * 3, t)) * env * overall * 0.7
    return generate_samples(dur, gen)


def gen_menu_select():
    """Short click/blip."""
    dur = 0.05
    def gen(t, d):
        freq = 1200
        env = envelope_linear(t, d, 0.002, 0.02)
        return square(freq, t, 0.25) * env * 0.6
    return generate_samples(dur, gen)


def gen_menu_confirm():
    """Confirmation beep - two quick tones."""
    dur = 0.15
    def gen(t, d):
        if t < 0.07:
            freq = 800
            env = envelope_linear(t, 0.07, 0.002, 0.01)
        else:
            freq = 1200
            env = envelope_linear(t - 0.07, 0.08, 0.002, 0.02)
        return (0.6 * square(freq, t, 0.25) + 0.4 * sine(freq, t)) * env * 0.65
    return generate_samples(dur, gen)


def gen_stage_clear():
    """Victory fanfare - triumphant ascending notes."""
    dur = 1.2
    # Fanfare: C E G C' (hold) E' G' C''
    notes = [
        (523.25, 0.12), (659.25, 0.12), (783.99, 0.12),
        (1046.50, 0.25), (1174.66, 0.12), (1318.51, 0.12),
        (1567.98, 0.35),
    ]
    def gen(t, d):
        # Find which note we're in
        cumulative = 0
        freq = notes[-1][0]
        local_t = 0
        current_note_dur = notes[-1][1]
        for nf, nd in notes:
            if t < cumulative + nd:
                freq = nf
                local_t = t - cumulative
                current_note_dur = nd
                break
            cumulative += nd
        else:
            local_t = t - cumulative
        env = envelope_adsr(local_t, current_note_dur, 0.005, 0.02, 0.7, 0.03)
        overall = envelope_linear(t, d, 0.005, 0.1)
        return (0.4 * square(freq, t, 0.25) + 0.3 * sine(freq, t) +
                0.2 * triangle(freq * 2, t) + 0.1 * sine(freq * 3, t)) * env * overall * 0.75
    return generate_samples(dur, gen)


# --------------- Music ---------------

def gen_music_title_theme():
    """Title theme - mysterious, iconic Galaga-style melody."""
    dur = 12.0  # loops
    bpm = 140
    beat = 60.0 / bpm

    # Melody notes (freq, start_beat, length_beats)
    melody_notes = [
        # Phrase 1
        (659.25, 0, 0.5), (783.99, 0.5, 0.5), (880, 1, 1), (783.99, 2, 0.5), (659.25, 2.5, 0.5),
        (587.33, 3, 1), (523.25, 4, 0.5), (587.33, 4.5, 0.5), (659.25, 5, 1), (523.25, 6, 1),
        (440, 7, 1),
        # Phrase 2
        (659.25, 8, 0.5), (783.99, 8.5, 0.5), (880, 9, 1), (1046.50, 10, 0.5), (880, 10.5, 0.5),
        (783.99, 11, 0.5), (659.25, 11.5, 0.5), (587.33, 12, 1), (523.25, 13, 0.5), (587.33, 13.5, 0.5),
        (659.25, 14, 1.5), (523.25, 15.5, 0.5),
        # Phrase 3 - repeat variation
        (440, 16, 0.5), (523.25, 16.5, 0.5), (587.33, 17, 1), (659.25, 18, 0.5), (587.33, 18.5, 0.5),
        (523.25, 19, 1), (440, 20, 0.5), (523.25, 20.5, 0.5), (587.33, 21, 1.5),
        (440, 22.5, 0.5), (392, 23, 1),
        # Phrase 4 - ending resolve
        (523.25, 24, 0.5), (587.33, 24.5, 0.5), (659.25, 25, 1), (783.99, 26, 0.5), (659.25, 26.5, 0.5),
        (587.33, 27, 1), (523.25, 28, 2),
    ]

    # Bass pattern
    bass_notes = [
        (130.81, 0, 2), (146.83, 2, 2), (164.81, 4, 2), (130.81, 6, 2),
        (130.81, 8, 2), (146.83, 10, 2), (164.81, 12, 2), (130.81, 14, 2),
        (110.00, 16, 2), (130.81, 18, 2), (110.00, 20, 2), (98.00, 22, 2),
        (130.81, 24, 2), (146.83, 26, 2), (164.81, 28, 2),
    ]

    total_beats = 30
    actual_dur = total_beats * beat
    if actual_dur < dur:
        dur = actual_dur

    n_samples = int(SAMPLE_RATE * dur)
    samples = [0.0] * n_samples

    for i in range(n_samples):
        t = i / SAMPLE_RATE
        current_beat = t / beat
        val = 0.0

        # Melody
        for freq, start_b, len_b in melody_notes:
            if start_b <= current_beat < start_b + len_b:
                local_t = (current_beat - start_b) * beat
                note_d = len_b * beat
                env = envelope_adsr(local_t, note_d, 0.01, 0.03, 0.6, 0.03)
                val += (0.4 * square(freq, t, 0.25) + 0.3 * triangle(freq, t) +
                        0.2 * sine(freq, t) + 0.1 * sine(freq * 2, t)) * env * 0.35
                break

        # Bass
        for freq, start_b, len_b in bass_notes:
            if start_b <= current_beat < start_b + len_b:
                local_t = (current_beat - start_b) * beat
                note_d = len_b * beat
                env = envelope_adsr(local_t, note_d, 0.01, 0.05, 0.5, 0.05)
                val += (0.5 * square(freq, t, 0.5) + 0.5 * sine(freq, t)) * env * 0.25
                break

        # Rhythmic hi-hat style
        sub_beat = (current_beat * 2) % 1.0
        if sub_beat < 0.1:
            val += noise() * 0.08 * (1 - sub_beat / 0.1)

        samples[i] = max(-1.0, min(1.0, val))

    return samples, dur


def gen_music_stage_start():
    """Stage start - short dramatic intro."""
    dur = 3.0
    bpm = 160
    beat = 60.0 / bpm

    notes = [
        (523.25, 0, 0.25), (523.25, 0.5, 0.25), (523.25, 1.0, 0.25),
        (659.25, 1.5, 0.5), (783.99, 2.0, 0.5), (1046.50, 2.5, 1.5),
    ]
    bass = [
        (130.81, 0, 1), (164.81, 1, 1), (196.00, 2, 1), (261.63, 3, 1),
    ]

    n_samples = int(SAMPLE_RATE * dur)
    samples = [0.0] * n_samples

    for i in range(n_samples):
        t = i / SAMPLE_RATE
        cb = t / beat
        val = 0.0

        for freq, sb, lb in notes:
            if sb <= cb < sb + lb:
                lt = (cb - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.005, 0.02, 0.7, 0.02)
                val += (0.5 * square(freq, t, 0.25) + 0.3 * sine(freq, t) +
                        0.2 * triangle(freq * 2, t)) * env * 0.4
                break

        for freq, sb, lb in bass:
            if sb <= cb < sb + lb:
                lt = (cb - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.01, 0.05, 0.5, 0.05)
                val += (0.6 * square(freq, t, 0.5) + 0.4 * sine(freq, t)) * env * 0.25
                break

        overall = envelope_linear(t, dur, 0.01, 0.2)
        samples[i] = max(-1.0, min(1.0, val * overall))

    return samples, dur


def gen_music_gameplay_loop():
    """Gameplay loop - driving rhythmic background music."""
    dur = 16.0
    bpm = 150
    beat = 60.0 / bpm

    # Driving bass line pattern (repeats every 8 beats)
    bass_pattern = [
        (130.81, 0, 0.5), (130.81, 1, 0.5), (146.83, 2, 0.5), (146.83, 3, 0.5),
        (164.81, 4, 0.5), (164.81, 5, 0.5), (146.83, 6, 0.5), (130.81, 7, 0.5),
    ]

    # Melody pattern (repeats every 16 beats)
    melody = [
        (523.25, 0, 0.5), (587.33, 0.5, 0.5), (659.25, 1, 1), (783.99, 2, 0.5),
        (659.25, 2.5, 0.5), (587.33, 3, 1),
        (523.25, 4, 0.5), (440, 4.5, 0.5), (523.25, 5, 1), (587.33, 6, 1), (523.25, 7, 1),
        # Second phrase
        (659.25, 8, 0.5), (783.99, 8.5, 0.5), (880, 9, 1), (783.99, 10, 0.5),
        (659.25, 10.5, 0.5), (587.33, 11, 1),
        (523.25, 12, 0.5), (587.33, 12.5, 0.5), (659.25, 13, 1), (587.33, 14, 1),
        (523.25, 15, 1),
    ]

    total_beats = int(dur / beat)
    n_samples = int(SAMPLE_RATE * dur)
    samples = [0.0] * n_samples

    for i in range(n_samples):
        t = i / SAMPLE_RATE
        cb = t / beat
        val = 0.0

        # Bass (repeats every 8 beats)
        cb_bass = cb % 8.0
        for freq, sb, lb in bass_pattern:
            if sb <= cb_bass < sb + lb:
                lt = (cb_bass - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.005, 0.03, 0.6, 0.02)
                val += (0.5 * square(freq, t, 0.5) + 0.5 * sine(freq, t)) * env * 0.25
                break

        # Melody
        cb_mel = cb % 16.0
        for freq, sb, lb in melody:
            if sb <= cb_mel < sb + lb:
                lt = (cb_mel - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.008, 0.02, 0.6, 0.02)
                val += (0.4 * square(freq, t, 0.25) + 0.3 * triangle(freq, t) +
                        0.2 * sine(freq, t) + 0.1 * sine(freq * 2, t)) * env * 0.3
                break

        # Kick on beats
        beat_pos = cb % 1.0
        if beat_pos < 0.08:
            kick_freq = 150 * (1 - beat_pos / 0.08) + 40
            val += sine(kick_freq, t) * (1 - beat_pos / 0.08) * 0.2

        # Hi-hat on off-beats
        hh_pos = (cb + 0.5) % 1.0
        if hh_pos < 0.05:
            val += noise() * (1 - hh_pos / 0.05) * 0.08

        overall = envelope_linear(t, dur, 0.05, 0.3)
        samples[i] = max(-1.0, min(1.0, val * overall))

    return samples, dur


def gen_music_challenging_stage():
    """Challenging stage - faster, more intense music."""
    dur = 10.0
    bpm = 180
    beat = 60.0 / bpm

    # Fast arpeggiated bass
    bass_pattern = [
        (196.00, 0, 0.25), (246.94, 0.25, 0.25), (293.66, 0.5, 0.25), (246.94, 0.75, 0.25),
        (220.00, 1, 0.25), (277.18, 1.25, 0.25), (329.63, 1.5, 0.25), (277.18, 1.75, 0.25),
        (261.63, 2, 0.25), (329.63, 2.25, 0.25), (392.00, 2.5, 0.25), (329.63, 2.75, 0.25),
        (246.94, 3, 0.25), (311.13, 3.25, 0.25), (369.99, 3.5, 0.25), (311.13, 3.75, 0.25),
    ]

    melody = [
        (783.99, 0, 0.5), (880, 0.5, 0.5), (1046.50, 1, 0.5), (880, 1.5, 0.5),
        (783.99, 2, 0.5), (659.25, 2.5, 0.5), (783.99, 3, 1),
    ]

    n_samples = int(SAMPLE_RATE * dur)
    samples = [0.0] * n_samples

    for i in range(n_samples):
        t = i / SAMPLE_RATE
        cb = t / beat
        val = 0.0

        cb_bass = cb % 4.0
        for freq, sb, lb in bass_pattern:
            if sb <= cb_bass < sb + lb:
                lt = (cb_bass - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.003, 0.02, 0.5, 0.01)
                val += (0.5 * square(freq, t, 0.3) + 0.5 * sine(freq, t)) * env * 0.25
                break

        cb_mel = cb % 4.0
        for freq, sb, lb in melody:
            if sb <= cb_mel < sb + lb:
                lt = (cb_mel - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.005, 0.02, 0.6, 0.02)
                val += (0.5 * square(freq, t, 0.25) + 0.3 * triangle(freq, t) +
                        0.2 * sine(freq * 2, t)) * env * 0.3
                break

        # Fast hi-hat
        hh_pos = (cb * 2) % 1.0
        if hh_pos < 0.04:
            val += noise() * (1 - hh_pos / 0.04) * 0.1

        overall = envelope_linear(t, dur, 0.03, 0.2)
        samples[i] = max(-1.0, min(1.0, val * overall))

    return samples, dur


def gen_music_fighter_captured():
    """Fighter captured - ominous, tense short piece."""
    dur = 4.0
    bpm = 100
    beat = 60.0 / bpm

    notes = [
        (220, 0, 1), (207.65, 1, 1), (196.00, 2, 1), (174.61, 3, 1.5),
    ]

    n_samples = int(SAMPLE_RATE * dur)
    samples = [0.0] * n_samples

    for i in range(n_samples):
        t = i / SAMPLE_RATE
        cb = t / beat
        val = 0.0

        for freq, sb, lb in notes:
            if sb <= cb < sb + lb:
                lt = (cb - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.02, 0.05, 0.7, 0.1)
                # Eerie vibrato
                vib = 1 + 0.02 * sine(5, t)
                val += (0.4 * square(freq * vib, t, 0.4) + 0.3 * sine(freq * vib, t) +
                        0.2 * sine(freq * 1.5 * vib, t) + 0.1 * triangle(freq * 0.5, t)) * env * 0.4
                break

        # Low rumble
        val += sine(55, t) * 0.1 * envelope_linear(t, dur, 0.1, 0.3)
        # Occasional noise
        if (cb * 4) % 1.0 < 0.03:
            val += noise() * 0.06

        overall = envelope_linear(t, dur, 0.05, 0.5)
        samples[i] = max(-1.0, min(1.0, val * overall))

    return samples, dur


def gen_music_perfect_bonus():
    """Perfect bonus - triumphant short fanfare."""
    dur = 4.0
    bpm = 160
    beat = 60.0 / bpm

    notes = [
        (659.25, 0, 0.5), (783.99, 0.5, 0.5), (880, 1, 0.5), (1046.50, 1.5, 0.5),
        (1174.66, 2, 0.5), (1318.51, 2.5, 0.5), (1567.98, 3, 1),
        (1046.50, 4, 0.5), (1318.51, 4.5, 0.5), (1567.98, 5, 1.5),
    ]

    bass = [
        (261.63, 0, 2), (329.63, 2, 2), (392.00, 4, 2),
    ]

    n_samples = int(SAMPLE_RATE * dur)
    samples = [0.0] * n_samples

    for i in range(n_samples):
        t = i / SAMPLE_RATE
        cb = t / beat
        val = 0.0

        for freq, sb, lb in notes:
            if sb <= cb < sb + lb:
                lt = (cb - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.005, 0.02, 0.7, 0.03)
                val += (0.4 * square(freq, t, 0.25) + 0.3 * sine(freq, t) +
                        0.2 * triangle(freq * 2, t) + 0.1 * sine(freq * 3, t)) * env * 0.35
                break

        for freq, sb, lb in bass:
            if sb <= cb < sb + lb:
                lt = (cb - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.01, 0.05, 0.5, 0.05)
                val += (0.5 * square(freq, t, 0.5) + 0.5 * sine(freq, t)) * env * 0.2
                break

        overall = envelope_linear(t, dur, 0.01, 0.3)
        samples[i] = max(-1.0, min(1.0, val * overall))

    return samples, dur


def gen_music_game_over():
    """Game over - sad, slow descending melody."""
    dur = 5.0
    bpm = 80
    beat = 60.0 / bpm

    notes = [
        (523.25, 0, 1), (493.88, 1, 1), (440.00, 2, 1), (392.00, 3, 1),
        (349.23, 4, 1), (329.63, 5, 1), (293.66, 6, 1.5), (261.63, 7.5, 2.5),
    ]

    n_samples = int(SAMPLE_RATE * dur)
    samples = [0.0] * n_samples

    for i in range(n_samples):
        t = i / SAMPLE_RATE
        cb = t / beat
        val = 0.0

        for freq, sb, lb in notes:
            if sb <= cb < sb + lb:
                lt = (cb - sb) * beat
                nd = lb * beat
                env = envelope_adsr(lt, nd, 0.02, 0.05, 0.6, 0.1)
                # Slight vibrato for sadness
                vib = 1 + 0.008 * sine(4.5, t)
                val += (0.4 * triangle(freq * vib, t) + 0.3 * sine(freq * vib, t) +
                        0.2 * square(freq * vib, t, 0.3) + 0.1 * sine(freq * 0.5, t)) * env * 0.4
                break

        # Fading low hum
        val += sine(65, t) * 0.05 * (1 - t / dur)

        overall = envelope_linear(t, dur, 0.02, 0.5)
        samples[i] = max(-1.0, min(1.0, val * overall))

    return samples, dur


def wav_to_ogg(wav_path, ogg_path):
    """Convert WAV to OGG using ffmpeg."""
    try:
        result = subprocess.run(
            ["ffmpeg", "-y", "-i", wav_path, "-c:a", "libvorbis", "-q:a", "4", ogg_path],
            capture_output=True, text=True, timeout=30
        )
        if result.returncode == 0:
            return True
        else:
            print(f"  ffmpeg error: {result.stderr[:200]}")
            return False
    except Exception as e:
        print(f"  ffmpeg exception: {e}")
        return False


def main():
    ensure_dirs()
    random.seed(42)  # Reproducible noise

    # ---- Sound Effects ----
    sfx = {
        "shoot.wav": gen_shoot,
        "enemy_shoot.wav": gen_enemy_shoot,
        "explosion_small.wav": gen_explosion_small,
        "explosion_large.wav": gen_explosion_large,
        "player_death.wav": gen_player_death,
        "tractor_beam.wav": gen_tractor_beam,
        "capture.wav": gen_capture,
        "rescue.wav": gen_rescue,
        "dual_join.wav": gen_dual_join,
        "dive_swoosh.wav": gen_dive_swoosh,
        "bonus.wav": gen_bonus,
        "extra_life.wav": gen_extra_life,
        "menu_select.wav": gen_menu_select,
        "menu_confirm.wav": gen_menu_confirm,
        "stage_clear.wav": gen_stage_clear,
    }

    print("=== Generating Sound Effects ===")
    for name, gen_func in sfx.items():
        path = os.path.join(SOUNDS_DIR, name)
        print(f"  Generating {name}...", end=" ", flush=True)
        random.seed(42)
        samples = gen_func()
        write_wav(path, samples)
        size = os.path.getsize(path)
        print(f"OK ({size:,} bytes, {len(samples)/SAMPLE_RATE:.2f}s)")

    # ---- Music ----
    music = {
        "title_theme": gen_music_title_theme,
        "stage_start": gen_music_stage_start,
        "gameplay_loop": gen_music_gameplay_loop,
        "challenging_stage": gen_music_challenging_stage,
        "fighter_captured": gen_music_fighter_captured,
        "perfect_bonus": gen_music_perfect_bonus,
        "game_over": gen_music_game_over,
    }

    print("\n=== Generating Music ===")
    ogg_success = True
    for name, gen_func in music.items():
        wav_path = os.path.join(MUSIC_DIR, f"{name}.wav")
        ogg_path = os.path.join(MUSIC_DIR, f"{name}.ogg")
        print(f"  Generating {name}...", end=" ", flush=True)
        random.seed(42)
        samples, dur = gen_func()
        write_wav(wav_path, samples)
        wav_size = os.path.getsize(wav_path)
        print(f"WAV OK ({wav_size:,} bytes, {dur:.1f}s)", end=" ", flush=True)

        # Convert to OGG
        if wav_to_ogg(wav_path, ogg_path):
            ogg_size = os.path.getsize(ogg_path)
            print(f"-> OGG OK ({ogg_size:,} bytes)")
            os.remove(wav_path)  # Clean up temp WAV
        else:
            print("-> OGG FAILED (keeping WAV)")
            ogg_success = False

    print("\n=== Summary ===")
    print(f"Sound effects: {SOUNDS_DIR}")
    for f in sorted(os.listdir(SOUNDS_DIR)):
        if f.endswith('.wav'):
            size = os.path.getsize(os.path.join(SOUNDS_DIR, f))
            print(f"  {f}: {size:,} bytes")

    print(f"\nMusic: {MUSIC_DIR}")
    for f in sorted(os.listdir(MUSIC_DIR)):
        if f.endswith(('.ogg', '.wav')):
            size = os.path.getsize(os.path.join(MUSIC_DIR, f))
            print(f"  {f}: {size:,} bytes")

    if not ogg_success:
        print("\nWARNING: Some OGG conversions failed. Music files left as WAV.")
        print("The game code expects .ogg files - you may need to update asset paths.")


if __name__ == "__main__":
    main()
