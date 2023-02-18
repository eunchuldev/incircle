use bevy_app::prelude::*;
use bevy_core::prelude::*;

use crate::{
    network::NetworkPlugin,
    physics::PhysicsPlugins,
    character::CharacterPlugin,
    language::LanguagePlugin,
};

use std::thread::sleep;
use std::time::{Instant, Duration};

fn my_runner(mut app: App) {
    const INTERVAL: Duration = Duration::from_micros(1_000_000 / 60);
    loop {
        let now = Instant::now();
        app.update();
        let elapsed = now.elapsed();
        if INTERVAL > elapsed {
            sleep(INTERVAL - elapsed);
        }
    }
}

pub fn app() -> App {
    let mut app = App::new();
    app
        .add_plugin(CorePlugin::default())
        .add_plugin(NetworkPlugin::default())
        .add_plugin(CharacterPlugin)
        .add_plugin(LanguagePlugin)
        .add_plugins(PhysicsPlugins)
        .set_runner(my_runner);
    app
}
