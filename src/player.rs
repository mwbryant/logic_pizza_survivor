use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_exp_start_pickup)
            .add_system(whip_attack)
            .add_system(player_gain_exp)
            .add_system(player_movement);
    }
}

fn player_exp_start_pickup(
    player: Query<(&Transform, &Collider), With<Player>>,
    rapier_context: Res<RapierContext>,
    mut orbs: Query<&mut ExpOrb>,
) {
    let (transform, collider) = player.single();

    rapier_context.intersections_with_shape(
        transform.translation.truncate(),
        0.0,
        collider,
        QueryFilter::new(),
        |entity| {
            if let Ok(mut orb) = orbs.get_mut(entity) {
                orb.collecting = true;
            }
            true
        },
    );
}

fn player_gain_exp(
    mut commands: Commands,
    orbs: Query<(Entity, &Transform, &ExpOrb)>,
    mut player: Query<(&Transform, &mut Player), Without<ExpOrb>>,
) {
    let (player_transform, mut player) = player.single_mut();

    for (entity, transform, orb) in &orbs {
        //TODO probably should use physics for this
        if Vec2::distance(
            transform.translation.truncate(),
            player_transform.translation.truncate(),
        ) < 0.3
        {
            //TODO event for sound
            player.exp += orb.value;
            commands.entity(entity).despawn_recursive();
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

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            SpriteBundle::default(),
            Player {
                exp: 0,
                next_level_exp: 5,
                level: 1,
                speed: 3.0,
                health: 100.0,
                max_health: 100.0,
            },
            Name::new("Player"),
            Collider::ball(1.0),
        ))
        .with_children(|commands| {
            commands.spawn((
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
            ));
        });
}

fn player_movement(
    mut player: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, player) = player.single_mut();
    if input.pressed(KeyCode::W) {
        transform.translation.y += time.delta_seconds() * player.speed;
    }
    if input.pressed(KeyCode::S) {
        transform.translation.y -= time.delta_seconds() * player.speed;
    }
    if input.pressed(KeyCode::A) {
        transform.translation.x -= time.delta_seconds() * player.speed;
    }
    if input.pressed(KeyCode::D) {
        transform.translation.x += time.delta_seconds() * player.speed;
    }
}
