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
        world.commands()
            .spawn(Name::from(f!("affect: {:?}", self.affect_type)))
            .insert(Affect(self.affect_type))
            .insert(AffectValue(self.value))
            .insert(Target(self.target))
            .insert(Sender(self.sender))
        ;
    }
}