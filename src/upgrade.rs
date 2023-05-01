use std::time::Duration;

use crate::{
    attack::{spawn_area_shot, spawn_close_shot, spawn_whip, whip_attack_facing},
    prelude::*,
};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                apply_player_upgrade,
                apply_whip_upgrade.after(whip_attack_facing),
                apply_area_shot_upgrade,
                apply_close_shot_upgrade,
            )
                .in_set(OnUpdate(GameState::Gameplay)),
        )
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
    assets: Res<AssetServer>,
    mut whips: Query<(&mut Whip, &mut Transform)>,
    player: Query<Entity, With<Player>>,
) {
    let player = player.single();

    for upgrade in reader.iter() {
        if &UpgradeSelected(WeaponUpgrade::Whip) == upgrade {
            info!("Upgrade whip");
            // Spawn Whip 1
            if whips.iter().count() == 0 {
                let whip_1 = spawn_whip(&mut commands, &assets);
                commands.entity(player).add_child(whip_1);
                return;
            }
            // Spawn Whip 2 and lock whip 1
            if let Ok((mut whip, mut transform)) = whips.get_single_mut() {
                *transform = Transform::from_xyz(-3.5, 0.0, 0.0);
                whip.timer.set_elapsed(Duration::from_secs_f32(0.3));

                let whip_2 = spawn_whip(&mut commands, &assets);
                commands.entity(player).add_child(whip_2);
            //Already has 2 whips, just buff damage
            } else {
                for (mut whip, _) in &mut whips {
                    whip.damage *= 1.10;
                }
            }
        }
    }
}

fn apply_close_shot_upgrade(
    mut commands: Commands,
    mut reader: EventReader<UpgradeSelected>,
    //TODO upgrade existing sometimes
    //mut shots: Query<&mut CloseShot>,
    player: Query<Entity, With<Player>>,
) {
    let player = player.single();

    for upgrade in reader.iter() {
        if &UpgradeSelected(WeaponUpgrade::CloseShot) == upgrade {
            // Spawn new close shot
            let close_shot = spawn_close_shot(&mut commands);
            commands.entity(player).add_child(close_shot);
        }
    }
}

fn apply_area_shot_upgrade(
    mut commands: Commands,
    mut reader: EventReader<UpgradeSelected>,
    //TODO upgrade existing sometimes
    //mut shots: Query<&mut AreaShot>,
    player: Query<Entity, With<Player>>,
) {
    let player = player.single();

    for upgrade in reader.iter() {
        if &UpgradeSelected(WeaponUpgrade::AreaShot) == upgrade {
            // Spawn new close shot
            let area_shot = spawn_area_shot(&mut commands);
            commands.entity(player).add_child(area_shot);
        }
    }
}
