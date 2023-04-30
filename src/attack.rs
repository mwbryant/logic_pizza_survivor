use bevy::utils::FloatOrd;

use crate::prelude::*;

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

pub fn spawn_area_shot(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Area Shot"),
            AreaShot {
                timer: Timer::from_seconds(2.5, TimerMode::Repeating),
            },
            RngComponent::new(),
        ))
        .id()
}

pub fn spawn_area_shot_bullet(commands: &mut Commands, spawn_pos: Vec2) -> Entity {
    info!("Spawning bullet");
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.0),
                sprite: Sprite {
                    color: Color::SEA_GREEN,
                    custom_size: Some(Vec2::new(2.5, 2.5)),
                    ..default()
                },
                ..default()
            },
            Name::new("Area Shot Bullet"),
            AreaShotBullet {
                lifetime: Timer::from_seconds(8.0, TimerMode::Once),
                damage_per_second: 2.0,
            },
            Sensor,
            Collider::ball(1.25),
        ))
        .id()
}

fn area_shot_bullet(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform, &Collider, &mut AreaShotBullet)>,
    rapier_context: Res<RapierContext>,
    mut enemy: Query<&mut Enemy>,
    time: Res<Time>,
) {
    for (bullet_entity, transform, collider, mut bullet) in &mut bullets {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.just_finished() {
            commands.entity(bullet_entity).despawn_recursive();
        }

        rapier_context.intersections_with_shape(
            transform.translation.truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok(mut enemy) = enemy.get_mut(entity) {
                    enemy.health -= bullet.damage_per_second * time.delta_seconds();
                }
                true
            },
        );
    }
}

pub fn spawn_close_shot(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Close Shot"),
            CloseShot {
                timer: Timer::from_seconds(0.7, TimerMode::Repeating),
            },
        ))
        .id()
}

pub fn spawn_close_shot_bullet(
    commands: &mut Commands,
    spawn_pos: Vec2,
    direction: Vec2,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.0),
                sprite: Sprite {
                    color: Color::ORANGE,
                    custom_size: Some(Vec2::new(0.2, 0.2)),
                    ..default()
                },
                ..default()
            },
            Name::new("Close Shot Bullet"),
            CloseShotBullet {
                lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                damage: 2.0,
                speed: 3.5,
                direction,
            },
            Sensor,
            Collider::cuboid(0.2, 0.2),
        ))
        .id()
}

pub fn spawn_whip(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(3.5, 0.0, 0.0),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(4.0, 0.6)),
                    ..default()
                },
                ..default()
            },
            Name::new("Whip"),
            Whip {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                damage: 5.0,
            },
            Sensor,
            Collider::cuboid(2.0, 0.3),
        ))
        .id()
}

fn whip_attack_facing(mut whips: Query<&mut Transform, With<Whip>>, player: Query<&Player>) {
    let player = player.single();

    if let Ok(mut whip) = whips.get_single_mut() {
        whip.translation = match player.facing {
            Facing::Left => Vec3::new(-3.5, 0.0, 0.0),
            Facing::Right => Vec3::new(3.5, 0.0, 0.0),
        };
    }
}

fn close_shot_bullet(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &Collider, &mut CloseShotBullet)>,
    rapier_context: Res<RapierContext>,
    mut enemy: Query<&mut Enemy>,
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
                if let Ok(mut enemy) = enemy.get_mut(entity) {
                    enemy.health -= bullet.damage;
                    commands.entity(bullet_entity).despawn_recursive();
                }
                true
            },
        );
    }
}

fn close_shot_attack(
    mut commands: Commands,
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
    time: Res<Time>,
) {
    for (transform, mut close_shot, mut rng) in &mut close_shots {
        close_shot.timer.tick(time.delta());
        if close_shot.timer.just_finished() {
            let location = Vec2::new(rng.f32_normalized(), rng.f32_normalized()).normalize() * 7.0;
            let offset = Vec2::new(rng.f32_normalized(), rng.f32_normalized()) * 1.5;

            spawn_area_shot_bullet(
                &mut commands,
                transform.translation().truncate() + location + offset,
            );
        }
    }
}

fn whip_attack(
    mut whips: Query<(&Collider, &GlobalTransform, &mut Whip, &mut Visibility)>,
    mut enemy: Query<(&mut Sprite, &mut Enemy)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, mut whip, mut visibility) in &mut whips {
        whip.timer.tick(time.delta());

        *visibility = if whip.timer.percent() < 0.1 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if whip.timer.just_finished() {
            rapier_context.intersections_with_shape(
                transform.translation().truncate(),
                0.0,
                collider,
                QueryFilter::new(),
                |entity| {
                    if let Ok((mut sprite, mut enemy)) = enemy.get_mut(entity) {
                        sprite.color = Color::PINK;
                        enemy.health -= whip.damage;
                    }
                    true
                },
            );
        }
    }
}
