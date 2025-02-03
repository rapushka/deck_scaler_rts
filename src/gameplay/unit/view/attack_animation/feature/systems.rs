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
    transforms: Query<&Transform>,
    global_transforms: Query<&GlobalTransform>,
) {
    for AttackCharged(attacker) in events.read() {
        let (unit, UnitHeadView(head), Opponent(opponent)) = cq!(units.get(*attacker));
        let transform = cq!(transforms.get(*head));

        let target = head.into_target();
        let mut transform = target.transform_state(*transform);

        let attacker_world_position = cq!(global_transforms.get(*head)).translation();
        let target_world_position = cq!(global_transforms.get(*opponent)).translation();
        let direction = (target_world_position - attacker_world_position).normalize();
        let offset = direction * 25.0;
        commands.entity(unit)
            .insert(PlayingAttackAnimation(ATTACK_ANIMATION_DURATION.to_timer()))
            .insert(TweenTarget)

            .animation()
            .insert(tween::sequence((
                DO(
                    0.05.to_seconds(),
                    EaseKind::BackOut,
                    transform.translation_by(offset),
                ),
                DO(
                    0.2.to_seconds(),
                    EaseKind::BackOut,
                    transform.translation_by(-offset),
                ),
            )))
        ;
    }
}

pub fn end_attack_animation(
    mut commands: Commands,
    mut units: Query<(Entity, &mut PlayingAttackAnimation)>,
    time: Res<Time>,
) {
    for (unit, mut animation) in units.iter_mut() {
        let timer = &mut animation.0;
        timer.tick(time.delta());

        if timer.just_finished() {
            commands.entity(unit)
                .remove::<PlayingAttackAnimation>()
            ;
        }
    }
}