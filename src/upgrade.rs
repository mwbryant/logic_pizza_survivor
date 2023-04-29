use std::time::Duration;

use crate::prelude::*;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_player_upgrade)
            .add_system(apply_whip_upgrade)
            .add_event::<UpgradeSelected>();
    }
}

fn apply_player_upgrade(mut reader: EventReader<UpgradeSelected>, mut player: Query<&mut Player>) {
    let mut player = player.single_mut();

    for upgrade in reader.iter() {
        match upgrade.0 {
            WeaponUpgrade::HealthUp => {
                let increase = player.max_health * 0.10;
                player.health += increase;
                player.max_health += increase;
            }
            WeaponUpgrade::SpeedUp => {
                player.speed *= 1.10;
            }
            _ => {}
        }
    }
}

//XXX messy function
fn apply_whip_upgrade(
    mut commands: Commands,
    mut reader: EventReader<UpgradeSelected>,
    mut whips: Query<(&mut Whip, &mut Transform, &Parent)>,
) {
    for _upgrade in reader.iter() {
        if matches!(UpgradeSelected(WeaponUpgrade::Whip), _upgrade) {
            if let Ok((mut whip, mut transform, parent)) = whips.get_single_mut() {
                *transform = Transform::from_xyz(-3.5, 0.0, 0.0);
                whip.timer.set_elapsed(Duration::from_secs_f32(0.3));
                //FIXME move so this can be kept in sync with whip 1
                let whip_2 = commands
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
                        Name::new("Whip2"),
                        Whip {
                            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                            damage: 5.0,
                        },
                        Sensor,
                        Collider::cuboid(2.0, 0.3),
                    ))
                    .id();

                commands.entity(**parent).add_child(whip_2);
            } else {
                for (mut whip, _, _) in &mut whips {
                    whip.damage *= 1.10;
                }
            }
        }
    }
}
