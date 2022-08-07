use crate::prelude::*;
use legion::systems::CommandBuffer;
use macroquad::rand::ChooseRandom;
use nanoserde::DeRon;

#[derive(Clone, Debug, DeRon)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: Vec<usize>,
    pub frequency: i32,
    pub name: String,
    pub sprite: Sprite,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Clone, DeRon, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, DeRon, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub async fn load() -> Self {
        let file = load_string("assets/template.ron")
            .await
            .expect("Failed opening file");
        DeRon::deserialize_ron(&file).expect("Unable to load templates")
    }

    pub fn spawn_entities(&self, ecs: &mut World, level: usize, spawn_points: &[Point]) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = available_entities.choose() {
                self.spawn_entity(pt, entity, &mut commands);
            }
        });

        commands.flush(ecs);
    }

    fn spawn_entity(
        &self,
        pt: &Point,
        template: &Template,
        commands: &mut legion::systems::CommandBuffer,
    ) {
        let entity = commands.push((
            pt.clone(),
            Render {
                color: WHITE,
                sprite: template.sprite,
            },
            Name(template.name.clone()),
        ));

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer {});
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap {}),
                    _ => {
                        eprintln!("Warning: we don't know how to provide {}", provides);
                    }
                });
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon {});
            }
        }
    }
}
