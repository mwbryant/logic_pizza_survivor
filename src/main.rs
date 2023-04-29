use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_turborand::prelude::*;

pub const WIDTH: f32 = 857.0;
pub const HEIGHT: f32 = 480.0;
pub const RENDER_WIDTH: f32 = 1920.;
pub const RENDER_HEIGHT: f32 = 1080.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pizza Survivor".into(),
                        resolution: (WIDTH, HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(RngPlugin::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player_ui)
        .add_system(player_movement)
        .add_system(whip_attack)
        .add_system(enemy_death_check)
        .add_system(player_exp_start_pickup)
        .add_system(player_exp_ui_sync)
        .add_system(player_gain_exp)
        .add_system(enemy_movement)
        .add_system(orb_move_to_player)
        .run();
}

#[derive(Component)]
pub struct Player {
    pub exp: i64,
    pub next_level_exp: i64,
    pub level: i64,
    pub speed: f32,
    pub health: f32,
}

#[derive(Component)]
pub struct Whip {
    pub timer: Timer,
    pub damage: f32,
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub health: f32,
}

#[derive(Resource)]
pub struct MainRender(pub Handle<Image>);

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 1920,
        height: 1080,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(20.0);
    camera.camera.target = RenderTarget::Image(image_handle.clone());

    commands.spawn((camera, MainCamera, UiCameraConfig { show_ui: true }));

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(16.0, 9.0))));

    let material_handle = materials.add(ColorMaterial {
        texture: Some(image_handle.clone()),
        ..default()
    });

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
        Name::new("Base Render"),
    ));

    commands.insert_resource(MainRender(image_handle));

    let mut camera = Camera2dBundle::default();
    camera.camera.order = 999;
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 16.0,
        min_height: 9.0,
    };

    commands.spawn((
        camera,
        post_processing_pass_layer,
        UiCameraConfig { show_ui: false },
    ));
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

fn player_exp_ui_sync(mut ui: Query<&mut Style, With<ExpUI>>, player: Query<&Player>) {
    let mut style = ui.single_mut();
    let player = player.single();

    let percent = player.exp as f32 / player.next_level_exp as f32;
    style.size.width = Val::Percent(percent * 100.0);
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

#[derive(Bundle)]
pub struct ExpOrbBundle {
    #[bundle]
    pub sprite: SpriteBundle,
    pub exp_orb: ExpOrb,
    pub collider: Collider,
    pub sensor: Sensor,
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

#[derive(Component)]
pub struct ExpOrb {
    pub value: i64,
    pub collection_speed: f32,
    pub collecting: bool,
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

#[derive(Component)]
pub struct HeaderBarUI;

#[derive(Component)]
pub struct ExpUI;

fn spawn_player_ui(mut commands: Commands) {
    let parent_node = (
        NodeBundle {
            style: Style {
                //XXX using Px here because UI isn't based on camera size, just window size
                size: Size::new(Val::Px(RENDER_WIDTH), Val::Px(RENDER_HEIGHT * 0.15)),
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

fn spawn_player(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands
        .spawn((
            SpriteBundle::default(),
            Player {
                exp: 0,
                next_level_exp: 5,
                level: 1,
                speed: 10.0,
                health: 100.0,
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

    for i in 0..10 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                transform: Transform::from_xyz(1.0 + 0.1 * i as f32, 1.0, 100.0),
                ..default()
            },
            Enemy {
                speed: 5.0,
                health: 5.0,
            },
            Name::new("Enemy"),
            RngComponent::from(&mut global_rng),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED_Z,
            Damping {
                linear_damping: 100.0,
                angular_damping: 1.0,
            },
            Collider::ball(1.0),
        ));
    }
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
