use crate::gameplay::affect::deal_damage::DealDamage;
use crate::gameplay::affect::feature::*;
use crate::gameplay::affect::types::AffectType;
use crate::prelude::*;

pub struct CreateAffect {
    pub affect_type: AffectType,
    pub value: f32,
    pub sender: Entity,
    pub target: Entity,
}

impl Command for CreateAffect {
    fn apply(self, world: &mut World) {
        let affect_type = self.affect_type;

        let mut commands = world.commands();
        let mut entity = commands.spawn(Name::from(f!("affect: {:?}", affect_type)));

        let mut entity = entity
            .insert(Affect(affect_type))
            .insert(AffectValue(self.value))
            .insert(Target(self.target))
            .insert(Sender(self.sender))
            .reborrow()
            ;

        match affect_type {
            AffectType::DealDamage => entity.insert(DealDamage),
        };
    }
}