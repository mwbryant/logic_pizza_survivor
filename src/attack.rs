use std::{f32::consts::PI, time::Duration};

use bevy::utils::FloatOrd;

use crate::{prelude::*, ui::spawn_world_text};

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                whip_attack_facing,
                whip_attack,
                close_shot_attack,
                close_shot_bullet,
                area_shot_attack,
                area_shot_bullet,
            )
                .in_set(OnUpdate(GameState::Gameplay)),
        );
    }
}

fn damage_enemy(
    commands: &mut Commands,
    //Gross but makes font loading easier
    assets: &AssetServer,
    enemy: &mut Enemy,
    position: &Transform,
    damage: f32,
) {
    spawn_world_text(
        commands,
        assets,
        position.translation.truncate(),
        &format!("{:?}", damage as i32),
    );

    enemy.health -= damage;
}

pub fn spawn_area_shot(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Area Shot"),
            AreaShot {
                timer: Timer::from_seconds(2.5, TimerMode::Repeating),
            },
            GamePlayEntity,
            RngComponent::new(),
        ))
        .id()
}

pub fn spawn_area_shot_bullet(
    commands: &mut Commands,
    assets: &AssetServer,
    spawn_pos: Vec2,
) -> Entity {
    info!("Spawning bullet");
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.3),
                texture: assets.load("nacho.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(128.0 * PIXEL_TO_WORLD)),
                    ..default()
                },
                ..default()
            },
            Name::new("Area Shot Bullet"),
            AreaShotBullet {
                timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                lifetime: Timer::from_seconds(8.0, TimerMode::Once),
                damage_per_second: 4.1,
            },
            Sensor,
            GamePlayEntity,
            Collider::ball(1.5),
        ))
        .id()
}

fn area_shot_bullet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut bullets: Query<(Entity, &Transform, &Collider, &mut AreaShotBullet)>,
    rapier_context: Res<RapierContext>,
    mut enemy: Query<(&mut Enemy, &Transform)>,
    time: Res<Time>,
) {
    for (bullet_entity, transform, collider, mut bullet) in &mut bullets {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.just_finished() {
            commands.entity(bullet_entity).despawn_recursive();
        }

        bullet.timer.tick(time.delta());
        if bullet.timer.just_finished() {
            rapier_context.intersections_with_shape(
                transform.translation.truncate(),
                0.0,
                collider,
                QueryFilter::new(),
                |entity| {
                    if let Ok((mut enemy, transform)) = enemy.get_mut(entity) {
                        damage_enemy(
                            &mut commands,
                            &assets,
                            &mut enemy,
                            transform,
                            bullet.damage_per_second * bullet.timer.duration().as_secs_f32(),
                        );
                    }
                    true
                },
            );
        }
    }
}

pub fn spawn_close_shot(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Close Shot"),
            CloseShot {
                timer: Timer::from_seconds(1.2, TimerMode::Repeating),
            },
        ))
        .id()
}

pub fn spawn_close_shot_bullet(
    commands: &mut Commands,
    assets: &AssetServer,
    spawn_pos: Vec2,
    direction: Vec2,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.0),
                texture: assets.load("burrito.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(PIXEL_TO_WORLD * 24.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("Close Shot Bullet"),
            CloseShotBullet {
                lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                damage: 2.0,
                speed: 4.5,
                direction,
            },
            Sensor,
            Collider::cuboid(0.2, 0.2),
        ))
        .id()
}

pub fn spawn_whip(commands: &mut Commands, assets: &AssetServer) -> Entity {
    let mut timer = Timer::from_seconds(2.0, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs(1));

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(3.5, 0.0, 0.0),
                texture: assets.load("ramen.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(156.0 * PIXEL_TO_WORLD, 33.0 * PIXEL_TO_WORLD)),
                    flip_x: true,
                    ..default()
                },
                ..default()
            },
            GamePlayEntity,
            Name::new("Whip"),
            Whip { timer, damage: 5.0 },
            TwoFrameAnimation {
                frame_1: assets.load("ramen.png"),
                frame_2: assets.load("ramen_2.png"),
                current_frame: false,
                //Jank
                timer: Timer::from_seconds(10000.0, TimerMode::Repeating),
            },
            Sensor,
            Collider::cuboid(156.0 * PIXEL_TO_WORLD / 2.0, 33.0 * PIXEL_TO_WORLD / 2.0),
        ))
        .id()
}

pub fn whip_attack_facing(
    mut whips: Query<(&mut Transform, &mut Sprite), With<Whip>>,
    player: Query<&Player>,
) {
    let player = player.single();

    if let Ok((mut whip, mut sprite)) = whips.get_single_mut() {
        whip.translation = match player.facing {
            Facing::Left => {
                sprite.flip_x = false;
                Vec3::new(-3.5, 0.0, 0.0)
            }
            Facing::Right => {
                sprite.flip_x = true;
                Vec3::new(3.5, 0.0, 0.0)
            }
        };
    }
}

fn close_shot_bullet(
    mut commands: Commands,
    //Gross but makes font loading easier
    assets: Res<AssetServer>,
    mut bullets: Query<(Entity, &mut Transform, &Collider, &mut CloseShotBullet), Without<Enemy>>,
    rapier_context: Res<RapierContext>,
    mut enemy: Query<(&mut Enemy, &Transform)>,
    time: Res<Time>,
) {
    for (bullet_entity, mut transform, collider, mut bullet) in &mut bullets {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.just_finished() {
            commands.entity(bullet_entity).despawn_recursive();
        }

        transform.translation += bullet.direction.extend(0.0) * time.delta_seconds() * bullet.speed;

        rapier_context.intersections_with_shape(
            transform.translation.truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok((mut enemy, transform)) = enemy.get_mut(entity) {
                    damage_enemy(&mut commands, &assets, &mut enemy, transform, bullet.damage);
                    commands.entity(bullet_entity).despawn_recursive();
                }
                true
            },
        );
    }
}

fn close_shot_attack(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut close_shots: Query<(&GlobalTransform, &mut CloseShot)>,
    enemy: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
) {
    for (transform, mut close_shot) in &mut close_shots {
        close_shot.timer.tick(time.delta());
        if close_shot.timer.just_finished() {
            if let Some(closest_enemy) = enemy.iter().min_by_key(|enemy_transform| {
                FloatOrd(Vec2::length(
                    transform.translation().truncate() - enemy_transform.translation.truncate(),
                ))
            }) {
                let direction = (closest_enemy.translation.truncate()
                    - transform.translation().truncate())
                .normalize();

                spawn_close_shot_bullet(
                    &mut commands,
                    &assets,
                    transform.translation().truncate(),
                    direction,
                );
            }
        }
    }
}

fn area_shot_attack(
    mut commands: Commands,
    mut close_shots: Query<(&GlobalTransform, &mut AreaShot, &mut RngComponent)>,
    assets: Res<AssetServer>,
    time: Res<Time>,
) {
    for (transform, mut close_shot, mut rng) in &mut close_shots {
        close_shot.timer.tick(time.delta());
        if close_shot.timer.just_finished() {
            let location = Vec2::new(rng.f32_normalized(), rng.f32_normalized()).normalize() * 7.0;
            let offset = Vec2::new(rng.f32_normalized(), rng.f32_normalized()) * 1.5;

            spawn_area_shot_bullet(
                &mut commands,
                &assets,
                transform.translation().truncate() + location + offset,
            );
        }
    }
}

fn whip_attack(
    mut commands: Commands,
    //Gross but makes font loading easier
    assets: Res<AssetServer>,
    mut whips: Query<(
        &Collider,
        &GlobalTransform,
        &mut Whip,
        &mut TwoFrameAnimation,
        &mut Visibility,
    )>,
    mut enemy: Query<(&mut Enemy, &Transform)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, mut whip, mut animation, mut visibility) in &mut whips {
        whip.timer.tick(time.delta());

        *visibility = if whip.timer.percent() < 0.2 || whip.timer.percent() > 0.9 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if whip.timer.percent() > 0.5 {
            animation.current_frame = true;
        }

        if whip.timer.just_finished() {
            animation.current_frame = false;
            rapier_context.intersections_with_shape(
                transform.translation().truncate(),
                0.0,
                collider,
                QueryFilter::new(),
                |entity| {
                    if let Ok((mut enemy, transform)) = enemy.get_mut(entity) {
                        damage_enemy(&mut commands, &assets, &mut enemy, transform, whip.damage);
                    }
                    true
                },
            );
        }
    }
}
