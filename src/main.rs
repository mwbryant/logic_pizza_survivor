use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use pizza_survivor::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pizza Survivor".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(RngPlugin::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(WaveManager {
            next_spawn: Timer::from_seconds(0.9, TimerMode::Repeating),
            wave_size: 1,
            to_spawn: Enemy {
                speed: 1.3,
                health: 5.0,
                damage_per_second: 10.0,
            },
        })
        .add_state::<GameState>()
        .add_plugin(UpgradePlugin)
        .add_plugin(ExpPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(spawn_background)
        .run();
}

fn spawn_background(mut commands: Commands, assets: Res<AssetServer>) {
    for i in -5..5 {
        for j in -5..5 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(i as f32 * 10.0, j as f32 * 20.0, 0.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(10.0, 20.0)),
                        ..default()
                    },
                    texture: assets.load("background.png"),
                    ..default()
                },
                Name::new("Background"),
            ));
        }
    }
}
