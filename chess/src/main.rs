#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use bevy::{prelude::*, window::PresentMode, log::LogPlugin, core::FrameCount, sprite::MaterialMesh2dBundle};
use tracing::Level;

use components::*;

mod components;

const DEFAULT_WINDOW_SIZE: (f32, f32) = (600., 500.);

fn main() {
  App::new()
    .insert_resource(Msaa::Sample8)
    .add_plugins(
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          visible: false,
          title: "Chess".into(),
          resolution: DEFAULT_WINDOW_SIZE.into(),
          present_mode: PresentMode::AutoVsync,
          resizable: false, // disable resizing for now, as it will need to be handled differently for sprites because i'm not smart enough
          enabled_buttons: bevy::window::EnabledButtons {
            maximize: false,
            ..Default::default()
          },
          ..default()
        }),
        ..default()
      }).set(LogPlugin {
        level: Level::INFO,
        filter: "chess=trace,naga=off,wgpu_hal=off,wgpu_core=off,bevy_render=error,bevy_app=error,bevy_ecs=error,bevy_asset=error".to_string(),
      }).set(ImagePlugin::default_nearest())
    )
    .add_systems(Startup, setup)
    .add_systems(First, reveal_window)
    .run();
}

fn run_system_once<M, S: IntoSystem<(), (), M> + 'static>(world: &mut World, system: S) {
  let id = world.register_system(system);
  let _ = world.run_system(id);
}

fn setup(world: &mut World) {
  run_system_once(world, create_camera);
  run_system_once(world, create_board);
}

fn create_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle {
    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    ..Default::default()
  });
}

fn create_board(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
  // window_query: Query<&mut Window>,
) {
  let size = 30.;
  let horiz = 3. / 2. * size;
  let vert = f32::sqrt(3.) * size;
  
  let mesh = meshes.add(shape::RegularPolygon::new(size, 6).into()).into();
  let white_material = materials.add(ColorMaterial::from(Color::hex("FFDBBF").unwrap()));
  let gray_material = materials.add(ColorMaterial::from(Color::hex("C28D78").unwrap()));
  let black_material = materials.add(ColorMaterial::from(Color::hex("784E3B").unwrap()));

  // let size = if let Ok(window) = window_query.get_single() {
  //   50. * f32::min(window.height() / DEFAULT_WINDOW_SIZE.0, window.width() / DEFAULT_WINDOW_SIZE.1)
  // } else {
  //   50.
  // };

  let offset = 3.5 * size; // offset down and left by 3.5 squares to perfectly center it. (half a square because the quads are centered on the corner, not the center)

  commands.spawn(SpriteBundle {
    sprite: Sprite {
      color: Color::hex("2D1B17").unwrap(),
      custom_size: Some((size * 8.25, size * 8.25).into()),
      ..Default::default()
    },
    transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
    ..Default::default()
  });

  let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L']; // hex chess conventionally skips 'J' for some strange reason
  for number in 1..=11 {
    let range = if number <= 6 { 
      0..letters.len()
    } else {
      (number - 6)..(letters.len() - (number - 6))
    };

    for letter in &letters[range] {
      print!("{}{}, ", letter, number);
    }
    println!();
  }

  for (rank, file) in itertools::iproduct!(0..8, 0..8) {
    let position = Vec3::new((rank as f32 * size) - offset, (file as f32 * size) - offset, 0.);
    let texture: Handle<Image> = if (rank + file) % 2 != 0 {
      asset_server.load("tile_white.png")
    } else {
      asset_server.load("tile_black.png")
    };

    commands.spawn(SpriteBundle {
      texture,
      sprite: Sprite {
        custom_size: Some((size, size).into()),
        ..Default::default()
      },
      transform: Transform::from_translation(position),
      ..Default::default()
    });
  }

  commands.spawn(MaterialMesh2dBundle {
    mesh,
    material: gray_material,
    transform: Transform::from_translation(Vec3::new(0., 0., 1.)).with_rotation(Quat::from_rotation_z(f32::to_radians(30.))),
    ..Default::default()
  });
}

fn reveal_window(
  mut window_query: Query<&mut Window>, frames: Res<FrameCount>
) {
  if frames.0 == 3 {
    for mut window in window_query.iter_mut() {
      window.visible = true;
    }
  }
}