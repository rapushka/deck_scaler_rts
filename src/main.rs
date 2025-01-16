use crate::assets::AssetsPlugin;
use crate::prelude::*;
use crate::third_party::ThirdPartyPlugin;
use crate::gameplay::GameplayPlugin;
use crate::camera::CameraPlugin;
use crate::infrastructure::InfrastructurePlugin;

mod gameplay;
mod prelude;
mod third_party;
mod camera;
mod assets;
mod infrastructure;

fn main() {
    App::new()
        .add_plugins(ThirdPartyPlugin)

        .add_plugins(InfrastructurePlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(CameraPlugin)

        .run();
}