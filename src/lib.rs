mod camera;
mod enemy;
mod exp;
mod player;
mod ui;

pub mod prelude {
    pub const WIDTH: f32 = 857.0;
    pub const HEIGHT: f32 = 480.0;
    pub const RENDER_WIDTH: f32 = 1920.;
    pub const RENDER_HEIGHT: f32 = 1080.;

    pub use crate::camera::GameCameraPlugin;
    pub use crate::enemy::EnemyPlugin;
    pub use crate::exp::ExpPlugin;
    pub use crate::player::PlayerPlugin;
    pub use crate::ui::GameUiPlugin;

    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use bevy_turborand::prelude::*;

    #[derive(Component)]
    pub struct Enemy {
        pub speed: f32,
        pub health: f32,
        pub damage_per_second: f32,
    }

    #[derive(Bundle)]
    pub struct ExpOrbBundle {
        #[bundle]
        pub sprite: SpriteBundle,
        pub exp_orb: ExpOrb,
        pub collider: Collider,
        pub sensor: Sensor,
    }

    #[derive(Component)]
    pub struct ExpOrb {
        pub value: i64,
        pub collection_speed: f32,
        pub collecting: bool,
    }

    #[derive(Resource)]
    pub struct MainRender(pub Handle<Image>);

    #[derive(Component)]
    pub struct MainCamera;

    #[derive(Component)]
    pub struct Player {
        pub exp: i64,
        pub next_level_exp: i64,
        pub level: i64,
        pub speed: f32,
        pub health: f32,
        pub max_health: f32,
    }

    #[derive(Component)]
    pub struct Whip {
        pub timer: Timer,
        pub damage: f32,
    }

    #[derive(Component)]
    pub struct HeaderBarUI;

    #[derive(Component)]
    pub struct PlayerUI;

    #[derive(Component)]
    pub struct ExpUI;

    #[derive(Component)]
    pub struct HealthUI;

    #[derive(Resource)]
    pub struct WaveManager {
        pub next_spawn: Timer,
    }
}
