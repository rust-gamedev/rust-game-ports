use crate::prelude::*;
// use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        for t in self.entities.iter() {
            if t.levels.contains(&level) {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            }
        }

        for pt in spawn_points.iter() {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pt, entity, ecs);
            }
        }
        // We don't need flushing; when manipulating World directly in Bevy, flushes are implicit.
    }

    fn spawn_entity(&self, pt: &Point, template: &Template, world: &mut World) {
        let mut world_spawner = world.spawn();
        let entity = world_spawner.insert_bundle((
            PointC(*pt),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));

        // In Bevy, we don't need to use commands to add components to the entity, since world.spawn().insert*()
        // returns a mutable entity reference.
        match template.entity_type {
            EntityType::Item => {
                entity.insert(Item {});
            }
            EntityType::Enemy => {
                entity.insert(Enemy {});
                entity.insert(FieldOfView::new(6));
                entity.insert(ChasingPlayer {});
                entity.insert(Health {
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap(),
                });
            }
        }
        if let Some(effects) = &template.provides {
            for (provides, n) in effects.iter() {
                match provides.as_str() {
                    "Healing" => {
                        entity.insert(ProvidesHealing { amount: *n });
                    }
                    "MagicMap" => {
                        entity.insert(ProvidesDungeonMap {});
                    }
                    _ => {
                        println!("Warning: we don't know how to provide {}", provides);
                    }
                }
            }
        }
    }
}
