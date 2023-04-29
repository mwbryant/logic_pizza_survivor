use crate::prelude::*;

pub struct ExpPlugin;

impl Plugin for ExpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(orb_move_to_player);
    }
}

impl Default for ExpOrbBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                sprite: Sprite {
                    color: Color::ALICE_BLUE,
                    custom_size: Some(Vec2::new(0.2, 0.2)),
                    ..default()
                },
                ..default()
            },
            exp_orb: ExpOrb {
                value: 1,
                collection_speed: 5.0,
                collecting: false,
            },
            collider: Collider::ball(1.0),
            sensor: Sensor,
        }
    }
}

fn orb_move_to_player(
    mut orbs: Query<(&mut Transform, &ExpOrb)>,
    player: Query<&Transform, (With<Player>, Without<ExpOrb>)>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for (mut transform, orb) in &mut orbs {
        if orb.collecting {
            //TODO bouncing animation
            let direction = (transform.translation.truncate()
                - player_transform.translation.truncate())
            .normalize();

            transform.translation -=
                (direction * time.delta_seconds() * orb.collection_speed).extend(0.);
        }
    }
}
