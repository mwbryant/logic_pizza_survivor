use crate::prelude::*;

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(two_frame_animation)
            .add_system(spawn_particles.in_schedule(OnEnter(GameState::LevelUp)))
            .add_system(despawn_particles.in_schedule(OnExit(GameState::LevelUp)))
            .add_system(update_particles.in_set(OnUpdate(GameState::LevelUp)));
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut rng: ResMut<GlobalRng>,
    coin_assets: Res<CoinAssets>,
    camera: Query<Entity, With<MainCamera>>,
) {
    let camera_transform = camera.single();

    for i in 0..500 {
        let x = rng.f32_normalized() * 20.0;
        let y = rng.f32_normalized() * 25.0 + 35.0;
        let child = commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, -120.0),
                    texture: coin_assets.image_1.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(0.4, 0.4)),
                        ..default()
                    },
                    ..default()
                },
                TwoFrameAnimation {
                    frame_1: coin_assets.image_1.clone(),
                    frame_2: coin_assets.image_2.clone(),
                    current_frame: false,
                    timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                },
                LevelUpParticle,
            ))
            .id();
        commands.entity(camera_transform).add_child(child);
    }
}

fn update_particles(
    mut particles: Query<&mut Transform, (With<LevelUpParticle>, Without<MainCamera>)>,
    time: Res<Time>,
) {
    for mut transform in &mut particles {
        transform.translation.y -= 10.0 * time.delta_seconds();
        if transform.translation.y < -10.0 {
            transform.translation.y = 10.0;
        }
    }
}

fn despawn_particles(mut commands: Commands, particles: Query<Entity, With<LevelUpParticle>>) {
    for entity in &particles {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn two_frame_animation(
    mut animated: Query<(&mut TwoFrameAnimation, &mut Handle<Image>)>,
    time: Res<Time>,
) {
    for (mut animation, mut image) in &mut animated {
        animation.timer.tick(time.delta());
        if animation.current_frame {
            *image = animation.frame_2.clone();
        } else {
            *image = animation.frame_1.clone();
        }

        if animation.timer.just_finished() {
            if animation.current_frame {
                animation.current_frame = false;
            } else {
                animation.current_frame = true;
            }
        }
    }
}
