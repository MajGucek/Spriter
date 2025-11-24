mod udp_layout;
mod udp_controller;
mod menu;
mod display;
mod button_system;

use crate::display::display_plugin;
use crate::menu::menu_plugin;
use crate::udp_controller::udp_controller_loop;
use crate::udp_layout::Telemetry;
use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::time::Fixed;
use bevy::window::WindowLevel;
use bevy::winit::{UpdateMode, WinitSettings};
use bevy_simple_text_input::TextInputPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Display,
}

#[derive(Resource, Default)]
struct TelemetryQueue {
    queue: Vec<Telemetry>,
}

#[derive(Resource)]
struct TelemetrySender(bevy_channel_trigger::ChannelSender<Telemetry>);



fn main() {
    use bevy_channel_trigger::ChannelTriggerApp;

    let mut app = App::new();
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: true,
                position: WindowPosition::At(IVec2 { x: 5, y: 40 }),
                window_level: WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            })
        )
            .add_plugins(TextInputPlugin)
            .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
            .insert_resource(TelemetryQueue::default())
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(menu_plugin)
        .add_plugins(display_plugin);

        let sender = app.add_channel_trigger::<Telemetry>();
    app.insert_resource(TelemetrySender(sender.clone()));

        std::thread::spawn(move || {
            udp_controller_loop(&sender);
        });

    app.add_observer(process_telemetry_packet);
    app.run();
}

fn process_telemetry_packet(
    trigger: On<Telemetry>,
    mut telemetry_queue: ResMut<TelemetryQueue>,
) {
    telemetry_queue.queue.push(*trigger.event());
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

