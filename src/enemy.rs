use crate::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_death_check)
            .add_system(enemy_movement)
            .add_system(spawn_enemy)
            .add_system(enemy_damage_player.in_base_set(CoreSet::PostUpdate));
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut wave_manager: ResMut<WaveManager>,
    player: Query<&Transform, With<Player>>,
    mut global_rng: ResMut<GlobalRng>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    wave_manager.next_spawn.tick(time.delta());

    if wave_manager.next_spawn.just_finished() {
        for i in 0..2 {
            // XXX is always off screen?
            let target_direction = 20.0
                * Vec2::new(global_rng.f32_normalized(), global_rng.f32_normalized()).normalize();

            let mut target_translation = target_direction.extend(100.0)
                //Add some jitter
                + Vec3::new(
                    global_rng.f32_normalized() ,
                    global_rng.f32_normalized(),
                    0.0,
                );

            target_translation += player_transform.translation.truncate().extend(0.0);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    transform: Transform::from_translation(target_translation),
                    ..default()
                },
                Enemy {
                    speed: 2.0,
                    health: 5.0,
                    damage_per_second: 10.0,
                },
                Name::new("Enemy"),
                RngComponent::from(&mut global_rng),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED_Z,
                Damping {
                    linear_damping: 100.0,
                    angular_damping: 1.0,
                },
                Collider::ball(0.8),
            ));
        }
    }
}

fn enemy_damage_player(
    enemies: Query<(&Collider, &GlobalTransform, &Enemy)>,
    mut player: Query<&mut Player>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, enemy) in &enemies {
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok(mut player) = player.get_mut(entity) {
                    player.health -= enemy.damage_per_second * time.delta_seconds();
                }
                true
            },
        );
    }
}

fn enemy_movement(
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    let player_transform = player.single();

    for (mut transform, enemy) in &mut enemy {
        let direction = (transform.translation.truncate()
            - player_transform.translation.truncate())
        .normalize();
        transform.translation -= (direction * time.delta_seconds() * enemy.speed).extend(0.);
    }
}

fn enemy_death_check(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform, &Enemy, &mut RngComponent)>,
) {
    //TODO dying animation
    for (entity, transform, enemy, mut rng) in &mut enemies {
        if enemy.health <= 0.0 {
            //TODO fire event for sounds
            commands.entity(entity).despawn_recursive();
            //Spawn exp orb (can extract into fn)
            if rng.f32() > 0.5 {
                let mut orb = ExpOrbBundle::default();
                orb.sprite.transform.translation.x = transform.translation.x;
                orb.sprite.transform.translation.y = transform.translation.y;
                commands.spawn(orb);
            }
        }
    }
}
