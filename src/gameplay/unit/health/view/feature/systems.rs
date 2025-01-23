use crate::gameplay::unit::health::view::*;
use crate::gameplay::unit::health::*;
use crate::gameplay::unit::view::UnitInfoContainer;

pub fn spawn_health_bars(
    mut commands: Commands,
    entities: Query<(Entity, &Health, &MaxHealth, &UnitInfoContainer), Without<HealthBar>>,
) {
    for (entity, health, max_health, container) in entities.iter() {
        let health = health.0;
        let max_health = max_health.0;

        let health_bar = commands.spawn(Name::new("Health Bar"))
            .set_parent(container.0)
            .insert(Text2d::new(f!("{health}/{max_health}")))
            .insert(TextColor::from(Color::Srgba(Srgba::new(0.75, 0.0, 0.0, 1.0))))
            .id();

        commands.entity(entity)
            .insert(HealthBar(health_bar))
        ;
    }
}

pub fn update_health_bars(
    units: Query<(&HealthBar, &Health, &MaxHealth)>,
    mut texts: Query<&mut Text2d>,
) {
    for (health_bar, health, max_health) in units.iter() {
        let health = health.0;
        let max_health = max_health.0;
        let mut health_bar_text = texts.get_mut(health_bar.0).expect("Health Bar must have Text");

        health_bar_text.0 = f!("{health}/{max_health}");
    }
}