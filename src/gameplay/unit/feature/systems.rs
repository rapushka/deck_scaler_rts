use crate::gameplay::unit::side::feature::OnSide;
use crate::prelude::*;

pub fn test_require_spawn_unit(
    mut events: EventWriter<SpawnUnit>,
) {
    events.send(SpawnUnit {
        id: UnitID::Crook,
        position: Vec2::new(0.0, -200.0),
        side: OnSide::Player,
        is_lead: true,
    });

    events.send(SpawnUnit {
        id: UnitID::Crook,
        position: Vec2::new(100.0, -200.0),
        side: OnSide::Player,
        is_lead: false,
    });

    // enemies
    events.send(SpawnUnit {
        id: UnitID::Rat,
        position: Vec2::new(100.0, 500.0),
        side: OnSide::Enemy,
        is_lead: false,
    });
}