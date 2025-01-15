use crate::prelude::*;
use crate::third_party::ThirdPartyPlugin;
use crate::gameplay::GameplayPlugin;
use crate::camera::CameraPlugin;

mod gameplay;
mod prelude;
mod third_party;
mod camera;

fn main() {
    App::new()
        .add_plugins(ThirdPartyPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
