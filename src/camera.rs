use crate::prelude::*;
use bevy::{
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
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(update_cursor)
            .init_resource::<CursorPosition>()
            .add_system(camera_follow.in_base_set(CoreSet::PostUpdate));
    }
}

fn update_cursor(
    mut cursor: ResMut<CursorPosition>,
    windows: Query<&Window>,
    final_camera: Query<&Camera, With<FinalCamera>>,
) {
    let camera = final_camera.single();
    let window = windows.single();

    if let Some(screen_position) = window.cursor_position() {
        let screen_position = screen_position / Vec2::new(window.width(), window.height());
        let screen_position = Vec2::new(screen_position.x, 1.0 - screen_position.y);
        info!("{:?}", screen_position);
        cursor.screen_position = screen_position;
    }
}

fn camera_follow(
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(player) = player.get_single() {
        let mut camera = camera.single_mut();
        camera.translation.x = player.translation.x;
        camera.translation.y = player.translation.y;
    }
}

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

    commands.spawn((camera, MainCamera, UiCameraConfig { show_ui: false }));

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
        FinalCamera,
        UiCameraConfig { show_ui: true },
    ));
}
