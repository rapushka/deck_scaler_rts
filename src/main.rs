use crate::assets::AssetsPlugin;
use crate::prelude::*;
use crate::third_party::ThirdPartyPlugin;
use crate::gameplay::GameplayPlugin;
use crate::camera::CameraPlugin;
use crate::common::CommonPlugin;
use crate::debug::DebugPlugin;
use crate::infrastructure::InfrastructurePlugin;
use crate::input::*;

mod gameplay;
mod prelude;
mod third_party;
mod camera;
mod assets;
mod infrastructure;
mod common;
mod input;
mod utils;
mod debug;

fn main() {
    App::new()
        .add_plugins(ThirdPartyPlugin)

        .add_plugins(InfrastructurePlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(CommonPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(DebugPlugin)

        .run();
}