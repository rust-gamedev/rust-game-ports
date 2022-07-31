use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    print_centered(1, "Cursor keys to move. G to pickup items.");
    bar_horizontal(
        Point::zero(),
        DISPLAY_WIDTH,
        player_health.current,
        player_health.max,
        RED,
        BLACK,
    );
    print_color_centered(
        0,
        format!("Health: {} / {}", player_health.current, player_health.max),
        WHITE,
    );

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();
    print_color_right(
        Point::new(SCREEN_WIDTH, 1),
        format!("Dungeon Level: {}", map_level + 1),
        YELLOW,
    );

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            print_pos(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });
    if y > 3 {
        print_color_pos(Point::new(3, 2), "Items carried", YELLOW);
    }
}
