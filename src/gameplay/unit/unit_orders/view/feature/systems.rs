use crate::gameplay::unit::unit_orders::view::feature::SpawnOrderViewCommand;
use crate::prelude::*;

pub fn view_target_position_order(
    mut commands: Commands,
    mut event: EventReader<ClickTargetPosition>,
) {
    for ClickTargetPosition(clicked_position) in event.read() {
        commands.queue(SpawnOrderViewCommand {
            position: *clicked_position,
            scale: 0.4,
            color: Srgba::GREEN,
        });
    }
}

pub fn view_target_unit_order(
    mut commands: Commands,
    mut event: EventReader<ClickTargetUnit>,
    transforms: Query<&Transform>,
) {
    for ClickTargetUnit(clicked_unit) in event.read() {
        let transform = transforms.get(*clicked_unit).expect("how?");

        commands.queue(SpawnOrderViewCommand {
            position: transform.translation.truncate(),
            scale: 1.0,
            color: Srgba::RED,
        });
    }
}
