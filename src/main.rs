use std::f32::consts::PI;

use bevy::{input::common_conditions::input_toggle_active, time::Stopwatch};
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
        //.add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(WaveManager {
            global_time: Stopwatch::new(),
            waves: vec![
                Wave {
                    next_spawn: Timer::from_seconds(0.9, TimerMode::Repeating),
                    wave_size: 3,
                    to_spawn: Enemy {
                        speed: 1.3,
                        health: 5.0,
                        damage_per_second: 10.0,
                    },
                },
                Wave {
                    next_spawn: Timer::from_seconds(0.2, TimerMode::Repeating),
                    wave_size: 1,
                    to_spawn: Enemy {
                        speed: 1.8,
                        health: 1.0,
                        damage_per_second: 3.0,
                    },
                },
                Wave {
                    next_spawn: Timer::from_seconds(10.0, TimerMode::Repeating),
                    wave_size: 10,
                    to_spawn: Enemy {
                        speed: 0.3,
                        health: 30.0,
                        damage_per_second: 10.0,
                    },
                },
            ],
        })
        .add_state::<GameState>()
        .add_plugin(UpgradePlugin)
        .add_plugin(ExpPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(GameAnimationPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(spawn_background)
        .add_system(advance_state.in_set(OnUpdate(GameState::StartingLoop)))
        .add_system(despawn_game_play.in_schedule(OnEnter(GameState::GameOver)))
        .run();
}

// Just to prevent on enter gameplay getting called every time after level up
fn advance_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Gameplay);
}

fn despawn_game_play(mut commands: Commands, entities: Query<Entity, With<GamePlayEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_background(mut commands: Commands, assets: Res<AssetServer>) {
    let size = 1080.0 * PIXEL_TO_WORLD;
    for i in -5..5 {
        for j in -5..5 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(i as f32 * size, j as f32 * size, 0.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(size, size)),
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
