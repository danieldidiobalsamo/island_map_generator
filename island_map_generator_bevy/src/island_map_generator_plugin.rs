use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use island_map_generator_algo::Generator;

pub struct IslandMapGeneratorPlugin;

impl Plugin for IslandMapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Settings>()
            .insert_resource(Settings {
                ..Default::default()
            })
            .add_systems(Startup, (spawn_camera, spawn_landspace).chain())
            .add_systems(Update, (generate_if_necessary, check_keyboard_input))
            .add_plugins(ResourceInspectorPlugin::<Settings>::new());
    }
}

#[derive(Resource, Reflect, InspectorOptions, Debug)]
#[reflect(Resource, InspectorOptions)]
struct Settings {
    #[inspector(min = 128, max = 1024, speed = 128.)]
    texture_resolution: usize,
    #[inspector(min = 1, max = 10, speed = 1.)]
    octaves: usize,
    #[inspector(min = 1., max = 10., speed = 0.1)]
    frequency: f64,
    #[inspector(min = 1., max = 10., speed = 0.1)]
    persistence: f64,
    #[inspector(min = 1., max = 10., speed = 0.1)]
    lacunarity: f64,
    #[inspector(min = -1., max = 1., speed = 0.1)]
    scale: f64,
    #[inspector(min = -1., max = 1., speed = 0.1)]
    bias: f64,
    #[inspector(min = 0, max = u32::MAX, speed = 1.)]
    seed: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            texture_resolution: 512,
            octaves: 6,
            frequency: 4.2,
            persistence: 1.5,
            lacunarity: 1.2,
            scale: 1.0,
            bias: 0.0,
            seed: 9000,
        }
    }
}

#[derive(Resource)]
struct LandscapeNoiseMap {
    material_handle: Handle<StandardMaterial>,
}

#[derive(Component, Debug)]
struct Landscape;

#[derive(Bundle)]
struct LandscapeBundle {
    landspace: Landscape,
    pbr: PbrBundle,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

fn spawn_landspace(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = generate_landscape_noise_map(&Settings::default());

    let plane = Mesh::from(shape::Quad {
        size: Vec2::new(10., 10.),
        ..default()
    });

    let image_handle = images.add(image);
    let plane_handle = meshes.add(plane);

    let plane_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        unlit: true,
        ..default()
    });
    commands.insert_resource(LandscapeNoiseMap {
        material_handle: plane_material_handle.clone(),
    });

    commands.spawn((LandscapeBundle {
        pbr: PbrBundle {
            mesh: plane_handle,
            material: plane_material_handle,
            ..default()
        },
        landspace: Landscape,
    },));
}

fn generate_landscape_noise_map(settings: &Settings) -> Image {
    let noise_map = Generator::new(
        (settings.texture_resolution, settings.texture_resolution),
        settings.octaves,
        settings.frequency,
        settings.persistence,
        settings.lacunarity,
        settings.scale,
        settings.bias,
        settings.seed,
    );

    let size = Extent3d {
        width: settings.texture_resolution as u32,
        height: settings.texture_resolution as u32,
        ..default()
    };

    Image::new(
        size,
        TextureDimension::D2,
        noise_map.get_noise_map_vec_rgba_8_u_norm(),
        TextureFormat::Rgba8Unorm,
    )
}

fn generate_if_necessary(
    settings: Res<Settings>,
    noise_map: ResMut<LandscapeNoiseMap>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if settings.is_changed() {
        let gen = Generator::new(
            (settings.texture_resolution, settings.texture_resolution),
            settings.octaves,
            settings.frequency,
            settings.persistence,
            settings.lacunarity,
            settings.scale,
            settings.bias,
            settings.seed,
        );

        let size = Extent3d {
            width: settings.texture_resolution as u32,
            height: settings.texture_resolution as u32,
            ..default()
        };

        if let Some(mat) = materials.get_mut(noise_map.material_handle.id()) {
            let old = mat.base_color_texture.clone().unwrap().id();

            mat.base_color_texture = Some(images.add(Image::new(
                size,
                TextureDimension::D2,
                gen.get_noise_map_vec_rgba_8_u_norm(),
                TextureFormat::Rgba8Unorm,
            )));

            images.remove(old);
        }
    }
}

fn check_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_events.send(bevy::app::AppExit);
    }
}
