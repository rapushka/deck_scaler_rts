use crate::gameplay::unit::attack::AttackCharged;
use crate::gameplay::unit::opponent::Opponent;
use crate::gameplay::unit::view::attack_animation::feature::PlayingAttackAnimation;
use crate::gameplay::unit::view::attack_animation::AttackAnimator;
use crate::gameplay::unit::view::UnitHeadView;
use crate::prelude::*;

pub const ATTACK_ANIMATION_DURATION: f32 = 1.0; // TODO: move outahere

pub fn play_attack_animation(
    mut commands: Commands,
    mut events: EventReader<AttackCharged>,
    units: Query<(Entity, &UnitHeadView, &AttackAnimator, &Opponent), Without<PlayingAttackAnimation>>,
    transforms: Query<&Transform>,
    global_transforms: Query<&GlobalTransform>,
) {
    for AttackCharged(attacker) in events.read() {
        let Ok((unit, head, animator, opponent)) = units.get(*attacker) else {
            continue;
        };

        let transform = transforms.get(head.0).unwrap();

        let target = head.0.into_target();
        let mut transform = target.transform_state(*transform);

        let attacker_world_position = global_transforms.get(unit).unwrap().translation();
        let target_world_position = global_transforms.get(opponent.0).unwrap().translation();
        let direction = (target_world_position - attacker_world_position).normalize();
        let offset = direction * 25.0;

        commands.entity(unit)
            .insert(PlayingAttackAnimation(ATTACK_ANIMATION_DURATION.to_timer()))
        ;

        commands.entity(animator.0)
            .insert(TweenTarget)

            .animation()
            .length(ATTACK_ANIMATION_DURATION.to_seconds())
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
    mut units: Query<(Entity, &mut PlayingAttackAnimation, &AttackAnimator)>,
    time: Res<Time<Virtual>>,
) {
    for (unit, mut animation, AttackAnimator(animator)) in units.iter_mut() {
        let timer = &mut animation.0;
        timer.tick(time.delta());

        if !timer.just_finished() {
            continue;
        }

        commands.entity(unit)
            .remove::<PlayingAttackAnimation>()
        ;

        commands.entity(*animator)
            .despawn_descendants()
        ;
    }
}