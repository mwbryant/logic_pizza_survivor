use crate::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_header_ui.in_schedule(OnEnter(GameState::StartingLoop)))
            .add_system(spawn_player_ui.in_schedule(OnEnter(GameState::StartingLoop)))
            .add_system(spawn_level_up_ui.in_schedule(OnEnter(GameState::LevelUp)))
            .add_system(despawn_level_up_ui.in_schedule(OnExit(GameState::LevelUp)))
            .add_system(spawn_main_menu_ui.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(despawn_main_menu_ui.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(spawn_game_over_ui.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(despawn_game_over_ui.in_schedule(OnExit(GameState::GameOver)))
            .add_system(level_up_button_system)
            .add_system(about_button_system)
            .add_system(about_back_button_system)
            .add_system(show_about_ui)
            .add_startup_system(spawn_about_ui)
            .add_system(start_button_system)
            .add_system(game_over_button_system)
            .add_system(update_world_text)
            .insert_resource(AboutShown(false))
            .add_systems(
                (player_health_ui_sync, player_exp_ui_sync).in_set(OnUpdate(GameState::Gameplay)),
            );
    }
}

fn update_world_text(
    mut commands: Commands,
    mut text: Query<(Entity, &mut Style, &mut WorldTextUI)>,
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    render_camera: Query<&Camera, With<FinalCamera>>,
    time: Res<Time>,
) {
    //AHHH
    let (camera, transform) = main_camera.single();
    let final_camera = render_camera.single();

    for (entity, mut style, mut world_ui) in &mut text {
        world_ui.lifetime.tick(time.delta());
        if world_ui.lifetime.just_finished() {
            commands.entity(entity).despawn_recursive();
        }

        world_ui.position = world_ui.position + world_ui.velocity * time.delta_seconds();

        if let Some(coords) = camera.world_to_viewport(transform, world_ui.position.extend(0.0)) {
            let mut coords = coords / Vec2::new(RENDER_WIDTH, RENDER_HEIGHT)
                * final_camera.logical_viewport_size().unwrap();
            coords.y = final_camera.logical_viewport_size().unwrap().y - coords.y;

            style.position = UiRect {
                top: Val::Px(coords.y),
                left: Val::Px(coords.x),
                bottom: Val::Px(coords.y),
                right: Val::Px(coords.x),
            }
        }
    }
}

pub fn spawn_world_text(commands: &mut Commands, assets: &AssetServer, position: Vec2, text: &str) {
    let font = assets.load("fonts/pointfree.ttf");

    //Gross offset because text is at top left of given coords
    let position = position + Vec2::new(-0.2, 1.4);

    let parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            z_index: ZIndex::Global(-100),
            ..default()
        },
        WorldTextUI {
            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
            velocity: Vec2::new(0.15, 1.5),
            position,
        },
    );

    let text = TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 32.0,
            color: Color::rgb(0.95, 0.2, 0.2),
        },
    );

    commands.spawn(parent).with_children(|commands| {
        commands.spawn(text);
    });
}

fn level_up_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &WeaponUpgrade),
        With<Button>,
    >,
    mut upgrade_event: EventWriter<UpgradeSelected>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, weapon) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                next_state.set(GameState::Gameplay);
                upgrade_event.send(UpgradeSelected(weapon.clone()));
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn start_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<StartButtonUI>),
    >,
    about: Res<AboutShown>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if about.0 {
        return;
    }
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                next_state.set(GameState::StartingLoop);
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn about_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<AboutButtonUI>),
    >,
    mut about: ResMut<AboutShown>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if about.0 {
        return;
    }
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                about.0 = true;
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn about_back_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<AboutBackButton>),
    >,
    mut about: ResMut<AboutShown>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !about.0 {
        return;
    }
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                about.0 = false;
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn show_about_ui(mut about: Query<&mut Visibility, With<AboutUI>>, about_shown: Res<AboutShown>) {
    for mut visible in &mut about {
        if about_shown.0 {
            *visible = Visibility::Visible;
        } else {
            *visible = Visibility::Hidden;
        }
    }
}

fn game_over_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<GameOverButtonUI>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::RED.into();
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *color = Color::GREEN.into();
            }
            Interaction::None => {
                *color = Color::DARK_GREEN.into();
            }
        }
    }
}

fn despawn_level_up_ui(mut commands: Commands, ui: Query<Entity, With<LevelUpUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn despawn_main_menu_ui(mut commands: Commands, ui: Query<Entity, With<MainMenuUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn despawn_game_over_ui(mut commands: Commands, ui: Query<Entity, With<GameOverUI>>) {
    for ui in &ui {
        commands.entity(ui).despawn_recursive();
    }
}

fn spawn_level_up_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<GlobalRng>,
) {
    let level_up_parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        LevelUpUI,
    );

    let level_up_popup = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(60.0), Val::Percent(40.0)),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
    };
    use strum::IntoEnumIterator;
    let upgrades = &WeaponUpgrade::iter().collect::<Vec<WeaponUpgrade>>();
    let option_1 = rng.sample(upgrades).unwrap();
    let mut option_2 = rng.sample(upgrades).unwrap();

    while option_1 == option_2 {
        option_2 = rng.sample(upgrades).unwrap();
    }

    commands.spawn(level_up_parent).with_children(|commands| {
        commands.spawn(level_up_popup).with_children(|commands| {
            spawn_button(commands, &asset_server, option_1);
            spawn_button(commands, &asset_server, option_2);
        });
    });
}

fn spawn_button(
    commands: &mut ChildBuilder,
    asset_server: &AssetServer,
    weapon: &WeaponUpgrade,
) -> Entity {
    let font = asset_server.load("fonts/pointfree.ttf");
    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(70.0), Val::Percent(35.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        },
        weapon.clone(),
    );

    let text = weapon.name();

    let button_text = TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );
    commands
        .spawn(button)
        .with_children(|commands| {
            commands.spawn(button_text);
        })
        .id()
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
            background_color: BackgroundColor(Color::DARK_GREEN),
            ..default()
        },
        GamePlayEntity,
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
                    top: Val::Percent(60.0),
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
        GamePlayEntity,
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

fn spawn_main_menu_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/pointfree.ttf");

    let menu_parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(38.0), Val::Percent(32.0)),
                position: UiRect {
                    left: Val::Percent(47.0),
                    right: Val::Auto,
                    top: Val::Percent(45.0),
                    bottom: Val::Auto,
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            //background_color: Color::DARK_GRAY.into(),
            ..default()
        },
        MainMenuUI,
    );

    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(70.0), Val::Percent(30.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        StartButtonUI,
    );

    let title_text = TextBundle::from_section(
        "DoorDash Survivor",
        TextStyle {
            font: font.clone(),
            font_size: 64.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let button_text = TextBundle::from_section(
        "Start Game!",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let button_2 = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(70.0), Val::Percent(30.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        AboutButtonUI,
    );

    let button_text_2 = TextBundle::from_section(
        "About",
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    commands.spawn(menu_parent).with_children(|commands| {
        commands.spawn(button).with_children(|commands| {
            commands.spawn(button_text);
        });
        commands.spawn(button_2).with_children(|commands| {
            commands.spawn(button_text_2);
        });
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            texture: assets.load("Menu2.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    333.0 * PIXEL_TO_WORLD * 3.86,
                    186.0 * PIXEL_TO_WORLD * 3.86,
                )),
                ..default()
            },
            ..default()
        },
        MainMenuUI,
    ));
}

fn spawn_game_over_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/pointfree.ttf");

    let menu_parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::left(Val::Percent(3.0)),
                ..default()
            },
            ..default()
        },
        GameOverUI,
    );

    let menu_title = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(70.0), Val::Percent(60.0)),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
    };

    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        GameOverButtonUI,
    );

    let title_text = TextBundle::from_section(
        "Game Over!",
        TextStyle {
            font: font.clone(),
            font_size: 64.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let button_text = TextBundle::from_section(
        "Back to Menu",
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    commands.spawn(menu_parent).with_children(|commands| {
        commands.spawn(menu_title).with_children(|commands| {
            commands.spawn(title_text);
            commands.spawn(button).with_children(|commands| {
                commands.spawn(button_text);
            });
        });
    });
}

fn spawn_about_ui(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/pointfree.ttf");

    let about_parent = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Global(100),
            ..default()
        },
        AboutUI,
    );

    let about_box = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(95.0), Val::Percent(95.0)),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
    };

    let button = (
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },

            background_color: Color::CRIMSON.into(),
            ..default()
        },
        AboutBackButton,
    );

    let font_size = 24.0;

    let text = vec![
        TextSection {
            value: "Delivery Survivor".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.9, 0.3, 0.3),
            },
        },
        TextSection {
            value: " is a action rougelike game designed for the".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        },
        TextSection {
            value: " LudumDare 53".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.3, 0.9, 0.3),
            },
        },
        TextSection {
            value: " game jam.  You play as a food delivery girl during the lunch rush at a college campus who must feed various students before they ambush you.\n\nYour food is ready when it is ready and will fire on it's own, all you can do is line up shots and survive.  Collect".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        },
        TextSection {
            value: " Money".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.1, 0.1, 0.9),
            },
        },
        TextSection {
            value: " to level up!\n\nThis game was devloped by".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        },
        TextSection {
            value: " Matthew Bryant (LogicProjects)".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.9, 0.2, 0.2),
            },
        },
        TextSection {
            value: " as part of a collaboration with ".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        },
        TextSection {
            value: "Johnny Thompson (Turbo Makes Games)".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.9, 0.2, 0.2),
            },
        },
        TextSection {
            value: ". Check them out on Youtube!".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        },
    ];

    let mut title_text = TextBundle::from_sections(text);

    // https://github.com/bevyengine/bevy/issues/1490
    title_text.style.size.width = Val::Px(WIDTH * 0.90);

    let button_text = TextBundle::from_section(
        "Back to Menu",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    let main_title_text = TextBundle::from_section(
        "About Delivery Survivor",
        TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );

    commands.spawn(about_parent).with_children(|commands| {
        commands.spawn(about_box).with_children(|commands| {
            commands.spawn(main_title_text);
            commands.spawn(title_text);
            commands.spawn(button).with_children(|commands| {
                commands.spawn(button_text);
            });
        });
    });
}
