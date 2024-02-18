use bevy::{
    prelude::*,
    window::{close_on_esc, EnabledButtons, PresentMode},
    winit::WinitWindows,
};
use winit::window::Icon;

pub struct GameConfPlugin;

impl Plugin for GameConfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Zombie Arena".into(),
                        resolution: (1000.0, 800.0).into(),
                        enabled_buttons: EnabledButtons {
                            maximize: false,
                            ..default()
                        },
                        resizable: false,
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        app.add_systems(Startup, set_window_icon)
            .add_systems(Update, close_on_esc);
    }
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon/icon.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
