use crate::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_header_ui)
            .add_startup_system(spawn_player_ui)
            .add_startup_system(spawn_level_up_ui)
            .add_system(button_system)
            .add_system(player_health_ui_sync)
            .add_system(player_exp_ui_sync);
    }
}

#[derive(Component)]
pub struct MyButton {
    size: Vec2,
}

fn button_system(cursor: Res<CursorPosition>, buttons: Query<(&MyButton, &GlobalTransform)>) {
    for (button, transform) in &buttons {
        let position = transform.translation().truncate() / Vec2::new(RENDER_WIDTH, RENDER_HEIGHT);
        if bevy::sprite::collide_aabb::collide(
            position.extend(0.0),
            button.size,
            cursor.screen_position.extend(0.0),
            Vec2::splat(0.01),
        )
        .is_some()
        {
            info!("On button");
        } else {
            info!("Off button");
        }
    }
}

fn spawn_level_up_ui(mut commands: Commands) {
    let level_up_parent = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };
    let level_up_popup = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(80.0), Val::Percent(70.0)),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::AQUAMARINE.into(),
        ..default()
    };

    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        },
        MyButton {
            size: Vec2::new(0.1, 0.1),
        },
    );

    commands.spawn(level_up_parent).with_children(|commands| {
        commands.spawn(level_up_popup).with_children(|commands| {
            commands.spawn(button);
        });
    });
}

fn player_health_ui_sync(mut ui: Query<&mut Style, With<HealthUI>>, player: Query<&Player>) {
    let mut style = ui.single_mut();
    let player = player.single();

    let percent = player.health / player.max_health;
    style.size.width = Val::Percent(percent * 100.0);
}

fn player_exp_ui_sync(mut ui: Query<&mut Style, With<ExpUI>>, player: Query<&Player>) {
    let mut style = ui.single_mut();
    let player = player.single();

    let percent = player.exp as f32 / player.next_level_exp as f32;
    style.size.width = Val::Percent(percent * 100.0);
}

fn spawn_header_ui(mut commands: Commands) {
    let parent_node = (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::GREEN),
            ..default()
        },
        HeaderBarUI,
        Name::new("Header Bar UI"),
    );

    let exp_node = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLUE),
            ..default()
        },
        ExpUI,
        Name::new("Exp UI"),
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(exp_node);
    });
}

fn spawn_player_ui(mut commands: Commands) {
    let parent_node = (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                size: Size::new(Val::Percent(5.0), Val::Percent(2.0)),
                position: UiRect {
                    //Player is always centered
                    left: Val::Percent(47.5),
                    right: Val::Auto,
                    top: Val::Percent(55.0),
                    bottom: Val::Auto,
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        PlayerUI,
        Name::new("Player UI"),
    );

    let health_node = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::RED),
            ..default()
        },
        HealthUI,
        Name::new("Health UI"),
    );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(health_node);
    });
}
