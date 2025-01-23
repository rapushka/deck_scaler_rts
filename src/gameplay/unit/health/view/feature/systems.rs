use crate::gameplay::unit::health::*;
use crate::gameplay::unit::health::view::*;

pub fn spawn_health_bar(
    mut commands: Commands,
    entities: Query<(Entity, &Health, &MaxHealth), Without<HealthBar>>,
) {
    for (entity, health, max_health) in entities.iter() {
        let health = health.0;
        let max_health = max_health.0;

        let health_bar = commands.spawn(Name::new("Health Bar"))
            .set_parent(entity)
            .insert(Text2d::new(f!("{health}/{max_health}")))
            .id();

        commands.entity(entity)
            .insert(HealthBar(health_bar))
        ;
    }
}