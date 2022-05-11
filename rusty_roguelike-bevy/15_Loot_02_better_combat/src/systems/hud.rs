use crate::components::Name;
use crate::prelude::*;

pub fn hud(
    player_query: Query<(Entity, &Player, &Health)>,
    item_query: Query<(&Name, &Carried), With<Item>>,
) {
    let (player_entity, player, player_health) = player_query.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    // The source project queries the player entity at this point, however, with the current (Bevy)
    // design, we don't really need it; we just modify the query we use in order to get the player entity
    // and health.

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", player.map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut y = 3;
    for (name, carried) in item_query.iter() {
        if carried.0 == player_entity {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        };
    }
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
