use crate::prelude::*;

pub fn assign_suit_component(
    mut commands: Commands,
    units: Query<(Entity, &UnitID), Without<Suit>>,
) {
    for (entity, unit_id) in units.iter() {
        let suit = get_suit(*unit_id);

        let mut entity = commands.entity(entity);
        entity
            .insert(suit)
        ;

        match suit {
            Suit::Spades => entity.insert(Spades),
            Suit::Hearts => entity.insert(Hearts),
            Suit::Clubs => entity.insert(Clubs),
            Suit::Diamonds => entity.insert(Diamonds),
        };
    }
}

fn get_suit(unit_id: UnitID) -> Suit { // TODO: unit config or something
    match unit_id {
        UnitID::Crook => Suit::Spades,
        UnitID::Rat => Suit::Clubs,
    }
}