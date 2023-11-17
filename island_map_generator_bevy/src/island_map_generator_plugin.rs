use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use island_map_generator_algo::Generator;

pub struct IslandMapGeneratorPlugin;

impl Plugin for IslandMapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Settings>()
            .insert_resource(Settings {
                ..Default::default()
            })
            .add_systems(Startup, (spawn_camera, spawn_landspace).chain())
            .add_systems(
                Update,
                (generate_if_necessary, check_keyboard_input, settings_menu),
            )
            .add_plugins(EguiPlugin);
    }
}

#[derive(Clone, Reflect, Debug, PartialEq)]
enum TextureQuality {
    VeryHigh = 2048,
    High = 1024,
    Medium = 512,
    Low = 256,
    VeryLow = 128,
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct Settings {
    texture_quality: TextureQuality,
    octaves: usize,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    scale: f64,
    bias: f64,
    seed: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            texture_quality: TextureQuality::Medium,
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

fn settings_menu(mut contexts: EguiContexts, mut settings: ResMut<Settings>) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut settings.texture_quality,
                TextureQuality::VeryLow,
                "VeryLow",
            );
            ui.radio_value(&mut settings.texture_quality, TextureQuality::Low, "Low");
            ui.radio_value(
                &mut settings.texture_quality,
                TextureQuality::Medium,
                "Medium",
            );
            ui.radio_value(&mut settings.texture_quality, TextureQuality::High, "High");
            ui.radio_value(
                &mut settings.texture_quality,
                TextureQuality::VeryHigh,
                "Very high",
            );
        });

        ui.add(
            egui::Slider::new(&mut settings.octaves, 0..=10)
                .text("Octaves")
                .drag_value_speed(1.0),
        );
        ui.add(
            egui::Slider::new(&mut settings.frequency, 1.0..=10.0)
                .text("Frequency")
                .drag_value_speed(0.1),
        );
        ui.add(
            egui::Slider::new(&mut settings.persistence, 1.2..=2.0)
                .text("persistence")
                .drag_value_speed(0.001),
        );
        ui.add(
            egui::Slider::new(&mut settings.lacunarity, 1.0..=1.5)
                .text("Lacunarity")
                .drag_value_speed(0.001),
        );
        ui.add(
            egui::Slider::new(&mut settings.scale, -1.0..=1.0)
                .text("scale")
                .drag_value_speed(0.001),
        );
        ui.add(
            egui::Slider::new(&mut settings.bias, -1.0..=1.0)
                .text("bias")
                .drag_value_speed(0.001),
        );
        ui.add(
            egui::Slider::new(&mut settings.seed, 0..=100000)
                .text("seed")
                .drag_value_speed(1.0),
        );
    });
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
    let image = generate_landscape_noise_map(Settings::default());

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

fn generate_landscape_noise_map(settings: Settings) -> Image {
    let texture_resolution = settings.texture_quality as usize;

    let noise_map = Generator::new(
        (texture_resolution, texture_resolution),
        settings.octaves,
        settings.frequency,
        settings.persistence,
        settings.lacunarity,
        settings.scale,
        settings.bias,
        settings.seed,
    );

    let size = Extent3d {
        width: texture_resolution as u32,
        height: texture_resolution as u32,
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
        let texture_resolution = settings.texture_quality.clone() as usize;
        let gen = Generator::new(
            (texture_resolution, texture_resolution),
            settings.octaves,
            settings.frequency,
            settings.persistence,
            settings.lacunarity,
            settings.scale,
            settings.bias,
            settings.seed,
        );

        let size = Extent3d {
            width: texture_resolution as u32,
            height: texture_resolution as u32,
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
