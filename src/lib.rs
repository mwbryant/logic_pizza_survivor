mod attack;
mod camera;
mod enemy;
mod exp;
mod player;
mod ui;
mod upgrade;

pub mod prelude {
    pub const WIDTH: f32 = 857.0;
    pub const HEIGHT: f32 = 480.0;
    pub const RENDER_WIDTH: f32 = 1920.;
    pub const RENDER_HEIGHT: f32 = 1080.;

    pub use crate::attack::AttackPlugin;
    pub use crate::camera::GameCameraPlugin;
    pub use crate::enemy::EnemyPlugin;
    pub use crate::exp::ExpPlugin;
    pub use crate::player::PlayerPlugin;
    pub use crate::ui::GameUiPlugin;
    pub use crate::upgrade::UpgradePlugin;

    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use bevy_turborand::prelude::*;

    #[derive(States, PartialEq, Eq, Default, Debug, Clone, Hash)]
    pub enum GameState {
        #[default]
        Gameplay,
        LevelUp,
    }

    #[derive(Component, Clone)]
    pub struct Enemy {
        pub speed: f32,
        pub health: f32,
        pub damage_per_second: f32,
    }

    #[derive(Resource, Default)]
    pub struct CursorPosition {
        pub screen_position: Vec2,
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
    pub struct FinalCamera;

    #[derive(Component)]
    pub struct LevelUpUI;

    #[derive(Component)]
    pub struct Player {
        pub exp: i64,
        pub next_level_exp: i64,
        pub level: i64,
        pub speed: f32,
        pub health: f32,
        pub max_health: f32,
        pub facing: Facing,
    }

    pub enum Facing {
        Left,
        Right,
    }

    #[derive(Component)]
    pub struct Whip {
        pub timer: Timer,
        pub damage: f32,
    }

    #[derive(Component)]
    pub struct CloseShot {
        pub timer: Timer,
    }

    #[derive(Component)]
    pub struct CloseShotBullet {
        pub lifetime: Timer,
        pub speed: f32,
        pub damage: f32,
        pub direction: Vec2,
    }

    #[derive(Component)]
    pub struct AreaShot {
        pub timer: Timer,
    }

    #[derive(Component)]
    pub struct AreaShotBullet {
        pub lifetime: Timer,
        pub damage_per_second: f32,
    }

    #[derive(Component, Clone)]
    pub enum WeaponUpgrade {
        Whip,
        CloseShot,
        AreaShot,
        HealthUp,
        SpeedUp,
    }

    pub struct UpgradeSelected(pub WeaponUpgrade);

    impl WeaponUpgrade {
        pub fn name(&self) -> &str {
            match self {
                WeaponUpgrade::Whip => "Whip Upgrade",
                WeaponUpgrade::CloseShot => "Close Shot Upgrade",
                WeaponUpgrade::AreaShot => "Area Shot Upgrade",
                WeaponUpgrade::HealthUp => "Health Up 10%",
                WeaponUpgrade::SpeedUp => "Speed Up 10%",
            }
        }
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
        pub wave_size: i32,
        pub to_spawn: Enemy,
    }
}
