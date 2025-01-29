use crate::gameplay::unit::attack::AttackCharged;
use crate::gameplay::unit::opponent::Opponent;
use crate::gameplay::unit::view::attack_animation::feature::PlayingAttackAnimation;
use crate::gameplay::unit::view::UnitHeadView;
use crate::prelude::*;

pub const ATTACK_ANIMATION_DURATION: f32 = 1.0; // TODO: move outahere

pub fn play_attack_animation(
    mut commands: Commands,
    mut events: EventReader<AttackCharged>,
    units: Query<(Entity, &UnitHeadView, &Opponent), Without<PlayingAttackAnimation>>, // TODO: blacklist PlayingAttackAnimation in other places
) {
    for AttackCharged(attacker) in events.read() {
        let (unit, UnitHeadView(head), Opponent(opponent)) = cq!(units.get(*attacker));

        info!("attacking");

        commands.entity(unit)
            .insert(PlayingAttackAnimation(Timer::from_seconds(ATTACK_ANIMATION_DURATION, TimerMode::Once)))
        ;
    }
}