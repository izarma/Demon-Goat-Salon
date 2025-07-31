use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::sample::Sample;
//use bevy_seedling::sample::Sample;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/imp/Imp-IDLE-Sprite-sheet.png")]
    pub imp_idle: Handle<Image>,
    #[asset(path = "images/imp/Imp-WALK-Sprite-sheet.png")]
    pub imp_walk: Handle<Image>,
    #[asset(path = "images/imp/Imp-JUMP-Sprite-sheet.png")]
    pub imp_jump: Handle<Image>,
    #[asset(path = "images/salon/control-panel.png")]
    pub control_panel: Handle<Image>,
    #[asset(path = "images/salon/background.png")]
    pub background: Handle<Image>,
    #[asset(path = "images/salon/floor.png")]
    pub floor: Handle<Image>,
    #[asset(path = "images/salon/platform.png")]
    pub moving_platform: Handle<Image>,
    #[asset(path = "images/salon/Golden-Apple.png")]
    pub golden_apple: Handle<Image>,
    #[asset(path = "images/salon/Backet-of-Gold-Apples.png")]
    pub backet_gold_apples: Handle<Image>,
    #[asset(path = "images/salon/joystick.png")]
    pub joystick: Handle<Image>,
    #[asset(path = "images/goat/goat-base.png")]
    pub goat_base: Handle<Image>,
    #[asset(path = "images/goat/goat-body.png")]
    pub goat_body: Handle<Image>,
    #[asset(path = "images/goat/goat-jaw.png")]
    pub goat_jaw: Handle<Image>,
    #[asset(path = "images/goat/goat-ears.png")]
    pub goat_ears: Handle<Image>,
    #[asset(path = "images/goat/goat-hair-A1-left.png")]
    pub goat_hair_a1_left: Handle<Image>,
    #[asset(path = "images/goat/goat-hair-A1-right.png")]
    pub goat_hair_a1_right: Handle<Image>,
    #[asset(path = "images/goat/goat-hair-A2-left.png")]
    pub goat_hair_a2_left: Handle<Image>,
    #[asset(path = "images/goat/goat-hair-A2-right.png")]
    pub goat_hair_a2_right: Handle<Image>,
    #[asset(path = "ui/game_over.png")]
    pub game_over_text: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/salon-background.ogg")]
    pub background: Handle<Sample>,
    #[asset(path = "audio/win.ogg")]
    pub win: Handle<Sample>,
    #[asset(path = "audio/lose.ogg")]
    pub lose: Handle<Sample>,
}
