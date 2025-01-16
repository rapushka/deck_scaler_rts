use crate::gameplay::unit::view::LoadingView;
use crate::prelude::*;

pub fn test_require_spawn_unit(
    mut events: EventWriter<SpawnUnit>,
) {
    events.send(SpawnUnit(UnitID::Crook));
}

pub fn spawn_unit(
    mut commands: Commands,
    mut events: EventReader<SpawnUnit>,
) {
    for spawn_unit in events.read() {
        let unit_id = spawn_unit.0.clone();

        commands.spawn(Name::from(f!("{unit_id:?}")))
            .insert(unit_id)
            .insert(LoadingView)
        ;
    }
}