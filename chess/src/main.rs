#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use bevy::{prelude::*, window::PresentMode, log::LogPlugin, core::FrameCount};
use tracing::Level;

const DEFAULT_WINDOW_SIZE: (f32, f32) = (500., 450.);

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
  asset_server: Res<AssetServer>,
  // window_query: Query<&mut Window>,
) {
  let size = 50.;

  // let size = if let Ok(window) = window_query.get_single() {
  //   50. * f32::min(window.height() / DEFAULT_WINDOW_SIZE.0, window.width() / DEFAULT_WINDOW_SIZE.1)
  // } else {
  //   50.
  // };

  let offset = 3.5 * size; // offset down and left by 3.5 squares to perfectly center it. (half a square because the quads are centered on the corner, not the center)

  for (i, j) in itertools::iproduct!(0..8, 0..8) {
    commands.spawn(SpriteBundle {
      texture: if (i + j + 1) % 2 == 0 {
        asset_server.load("tile_white.png")
      } else {
        asset_server.load("tile_black.png")
      },
      sprite: Sprite {
        custom_size: Some((size, size).into()),
        ..Default::default()
      },
      transform: Transform::from_translation(Vec3::new((i as f32 * size) - offset, (j as f32 * size) - offset, 0.)),
      ..Default::default()
    });
  }
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