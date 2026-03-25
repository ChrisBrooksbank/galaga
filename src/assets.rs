use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::AudioSource;

/// All game assets loaded during the Loading state.
/// Populated by bevy_asset_loader; available as a Resource once Loading completes.
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // --- Sprites ---

    #[asset(path = "sprites/player.png")]
    pub player_image: Handle<Image>,

    #[asset(path = "sprites/enemies.png")]
    pub enemies_image: Handle<Image>,

    /// Texture atlas layout for enemies.png: 8 columns × 4 rows of 16×16 sprites.
    /// Row 0: Boss (green×2, purple×2), Butterfly (×2), Bee (×2)
    /// Row 1: Scorpion (×2), Stingray (×2), Flagship, Flag-1, Flag-5, Flag-10
    /// Row 2: Explosion frames 1-6, Beam frames 1-2
    /// Row 3: Beam frames 3-4, Flag-20, Flag-30, Flag-50, (empty×3)
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 8, rows = 4))]
    pub enemies_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "sprites/bullets.png")]
    pub bullets_image: Handle<Image>,

    #[asset(path = "sprites/tractor_beam.png")]
    pub tractor_beam_image: Handle<Image>,

    #[asset(path = "sprites/stage_flags.png")]
    pub stage_flags_image: Handle<Image>,

    // --- Font ---

    #[asset(path = "fonts/PressStart2P-Regular.ttf")]
    pub font: Handle<Font>,

    // --- Sound Effects ---

    #[asset(path = "sounds/shoot.wav")]
    pub sfx_shoot: Handle<AudioSource>,

    #[asset(path = "sounds/enemy_shoot.wav")]
    pub sfx_enemy_shoot: Handle<AudioSource>,

    #[asset(path = "sounds/explosion_small.wav")]
    pub sfx_explosion_small: Handle<AudioSource>,

    #[asset(path = "sounds/explosion_large.wav")]
    pub sfx_explosion_large: Handle<AudioSource>,

    #[asset(path = "sounds/player_death.wav")]
    pub sfx_player_death: Handle<AudioSource>,

    #[asset(path = "sounds/tractor_beam.wav")]
    pub sfx_tractor_beam: Handle<AudioSource>,

    #[asset(path = "sounds/capture.wav")]
    pub sfx_capture: Handle<AudioSource>,

    #[asset(path = "sounds/rescue.wav")]
    pub sfx_rescue: Handle<AudioSource>,

    #[asset(path = "sounds/dual_join.wav")]
    pub sfx_dual_join: Handle<AudioSource>,

    #[asset(path = "sounds/dive_swoosh.wav")]
    pub sfx_dive_swoosh: Handle<AudioSource>,

    #[asset(path = "sounds/bonus.wav")]
    pub sfx_bonus: Handle<AudioSource>,

    #[asset(path = "sounds/extra_life.wav")]
    pub sfx_extra_life: Handle<AudioSource>,

    #[asset(path = "sounds/menu_select.wav")]
    pub sfx_menu_select: Handle<AudioSource>,

    #[asset(path = "sounds/menu_confirm.wav")]
    pub sfx_menu_confirm: Handle<AudioSource>,

    #[asset(path = "sounds/stage_clear.wav")]
    pub sfx_stage_clear: Handle<AudioSource>,

    // --- Music ---

    #[asset(path = "music/title_theme.ogg")]
    pub music_title_theme: Handle<AudioSource>,

    #[asset(path = "music/stage_start.ogg")]
    pub music_stage_start: Handle<AudioSource>,

    #[asset(path = "music/gameplay_loop.ogg")]
    pub music_gameplay_loop: Handle<AudioSource>,

    #[asset(path = "music/challenging_stage.ogg")]
    pub music_challenging_stage: Handle<AudioSource>,

    #[asset(path = "music/fighter_captured.ogg")]
    pub music_fighter_captured: Handle<AudioSource>,

    #[asset(path = "music/perfect_bonus.ogg")]
    pub music_perfect_bonus: Handle<AudioSource>,

    #[asset(path = "music/game_over.ogg")]
    pub music_game_over: Handle<AudioSource>,
}

/// Sprite atlas indices for enemies.png (8 columns × 4 rows, 16×16 per tile).
pub mod enemy_sprite_index {
    // Row 0
    pub const BOSS_GREEN_1: usize = 0;
    pub const BOSS_GREEN_2: usize = 1;
    pub const BOSS_PURPLE_1: usize = 2;
    pub const BOSS_PURPLE_2: usize = 3;
    pub const BUTTERFLY_1: usize = 4;
    pub const BUTTERFLY_2: usize = 5;
    pub const BEE_1: usize = 6;
    pub const BEE_2: usize = 7;
    // Row 1
    pub const SCORPION_1: usize = 8;
    pub const SCORPION_2: usize = 9;
    pub const STINGRAY_1: usize = 10;
    pub const STINGRAY_2: usize = 11;
    pub const FLAGSHIP: usize = 12;
    pub const FLAG_1: usize = 13;
    pub const FLAG_5: usize = 14;
    pub const FLAG_10: usize = 15;
    // Row 2
    pub const EXPLOSION_1: usize = 16;
    pub const EXPLOSION_2: usize = 17;
    pub const EXPLOSION_3: usize = 18;
    pub const EXPLOSION_4: usize = 19;
    pub const EXPLOSION_5: usize = 20;
    pub const EXPLOSION_6: usize = 21;
    pub const BEAM_1: usize = 22;
    pub const BEAM_2: usize = 23;
    // Row 3
    pub const BEAM_3: usize = 24;
    pub const BEAM_4: usize = 25;
    pub const FLAG_20: usize = 26;
    pub const FLAG_30: usize = 27;
    pub const FLAG_50: usize = 28;
}
