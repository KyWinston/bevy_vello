mod ui;

use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::asset::{embedded_asset, AssetMetaCheck};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_vello::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(VelloPlugin)
        .init_resource::<EmbeddedAssetRegistry>()
        .add_plugins(bevy_pancam::PanCamPlugin)
        .add_systems(Startup, setup_vector_graphics)
        .add_systems(Update, (print_metadata, ui::controls_ui));
    embedded_asset!(app, "assets/calendar.json");
    app.run();
}

fn setup_vector_graphics(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), bevy_pancam::PanCam::default()));
    commands
        .spawn(VelloAssetBundle {
            vector: asset_server.load::<VelloAsset>("embedded://demo/assets/calendar.json"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_scale(Vec3::splat(20.0)),
            debug_visualizations: DebugVisualizations::Visible,
            ..default()
        })
        .insert(
            LottiePlayer::new("stopped")
                .with_state({
                    PlayerState::new("stopped")
                        .playback_options(PlaybackOptions {
                            autoplay: false,
                            ..default()
                        })
                        .theme(Theme::new().add("calendar", Color::BLUE))
                        .transition(PlayerTransition::OnMouseEnter { state: "play" })
                        .reset_playhead_on_start()
                })
                .with_state(
                    PlayerState::new("play")
                        .playback_options(PlaybackOptions {
                            looping: PlaybackLoopBehavior::DoNotLoop,
                            speed: 0.25,
                            ..default()
                        })
                        .theme(Theme::new().add("calendar", Color::GREEN))
                        .transition(PlayerTransition::OnMouseLeave { state: "rev" }),
                )
                .with_state(
                    PlayerState::new("rev")
                        .playback_options(PlaybackOptions {
                            looping: PlaybackLoopBehavior::DoNotLoop,
                            direction: PlaybackDirection::Reverse,
                            speed: 0.25,
                            ..default()
                        })
                        .theme(Theme::new().add("calendar", Color::RED))
                        .transition(PlayerTransition::OnMouseEnter { state: "play" })
                        .transition(PlayerTransition::OnComplete { state: "stopped" }),
                ),
        );
}

fn print_metadata(
    mut asset_ev: EventReader<AssetEvent<VelloAsset>>,
    assets: Res<Assets<VelloAsset>>,
) {
    for ev in asset_ev.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            let asset = assets.get(*id).unwrap();
            if let Some(metadata) = asset.metadata() {
                info!(
                    "Animated asset loaded. Layers:\n{:#?}",
                    metadata.get_layers().collect::<Vec<_>>()
                );
            }
        }
    }
}
