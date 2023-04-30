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

//XXX messy function
fn apply_whip_upgrade(
    mut commands: Commands,
    mut reader: EventReader<UpgradeSelected>,
    mut whips: Query<(&mut Whip, &mut Transform)>,
    player: Query<Entity, With<Player>>,
) {
    let player = player.single();

    for _upgrade in reader.iter() {
        if matches!(UpgradeSelected(WeaponUpgrade::Whip), _upgrade) {
            if whips.iter().count() == 0 {
                // Spawn Whip 1
                let whip_1 = spawn_whip(&mut commands);
                commands.entity(player).add_child(whip_1);
                return;
            }
            if let Ok((mut whip, mut transform)) = whips.get_single_mut() {
                // Spawn Whip 2 and lock whip 1
                *transform = Transform::from_xyz(-3.5, 0.0, 0.0);
                whip.timer.set_elapsed(Duration::from_secs_f32(0.3));

                let whip_2 = spawn_whip(&mut commands);
                commands.entity(player).add_child(whip_2);
            } else {
                //Already has 2 whips, just buff damage
                for (mut whip, _) in &mut whips {
                    whip.damage *= 1.10;
                }
            }
        }
    }
}
