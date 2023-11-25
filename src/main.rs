use macroquad::math::Vec2;
use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

struct Player {
    collider: Actor,
    speed: Vec2,
}

struct Platform {
    collider: Solid,
    speed: f32,
}

pub fn draw_segment(x: f32, y: f32, radius: f32, rotation: f32, color: Color) {
    let rot = rotation.to_radians();
    let mut prev = Default::default();
    for i in 0..(5 + 1) {
        let rx = ((i as f32 * std::f32::consts::PI * 2.) / 20. + rot).cos();
        let ry = ((i as f32 * std::f32::consts::PI * 2.) / 20. + rot).sin();

        if i != 0 {
            draw_triangle(
                Vec2::new(x, y),
                prev,
                Vec2::new(x + radius * rx, y + radius * ry),
                if i % 2 == 0 { color } else { MAGENTA },
            );
        }
        prev = Vec2::new(x + radius * rx, y + radius * ry);
    }
}

fn rotate(p: Vec2, c: Vec2, angle: f32) -> Vec2 {
    let angle = angle.to_radians();
    Vec2::new(
        angle.cos() * (p.x - c.x) - angle.sin() * (p.y - c.y) + c.x,
        angle.sin() * (p.x - c.x) + angle.cos() * (p.y - c.y) + c.y,
    )
}

#[macroquad::main("Platformer")]
async fn main() {
    let tileset_f = load_texture("tileset.png").await;
    let tiled_map_json_f = load_string("map.json").await;
    let mut tileset = Texture2D::empty();
    let mut tiled_map_json = Default::default();
    let mut world = None;
    let mut player = None;
    let mut platform = None;
    let mut tiled_map = None;
    let main_layer_width = 40;
    let tileset_e = match tileset_f {
        Ok(v) => {
            tileset = v;
            Ok(())
        }
        Err(e) => Err(format!("{e}")),
    };
    let tiled_map_json_e = match tiled_map_json_f {
        Ok(v) => {
            tiled_map_json = v;
            Ok(())
        }
        Err(e) => Err(format!("{e}")),
    };

    let mut r = Ok(());
    if tileset_e.is_err() {
        r = tileset_e;
    } else if tiled_map_json_e.is_err() {
        r = tiled_map_json_e;
    }

    let mut text0 = match &r {
        Ok(_) => "No panic".to_string(),
        Err(e) => format!("Panic: {e}"),
    };

    let mut touched;
    let mut touch_start = Default::default();
    let mut up_touch = false;
    let mut down_touch = false;
    let mut left_touch = false;
    let mut right_touch = false;

    if r.is_ok() {
        tileset.set_filter(FilterMode::Nearest);
        let tiled_map_l =
            tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();
        let mut static_colliders = vec![];
        for (_x, _y, tile) in tiled_map_l.tiles("main layer", None) {
            static_colliders.push(if tile.is_some() {
                Tile::Solid
            } else {
                Tile::Empty
            });
        }

        let mut world_l = World::new();
        let tile_width = screen_width() / main_layer_width as f32;

        world_l.add_static_tiled_layer(
            static_colliders,
            tile_width,
            tile_width,
            main_layer_width,
            1,
        );

        // Don't know why player was at 50, 80, so I just calculate same position for resized world
        let x_proportion = 50.0 / 320.0;
        let y_proportion = 80.0 / 152.0;
        let x = x_proportion * (tile_width * main_layer_width as f32);
        let y = y_proportion * (19.0 * tile_width);
        let player_l = Player {
            collider: world_l.add_actor(vec2(x, y), tile_width as i32, tile_width as i32),
            speed: vec2(0., 0.),
        };

        let x_proportion = 170.0 / 320.0;
        let y_proportion = 130.0 / 152.0;
        let x = x_proportion * (tile_width * main_layer_width as f32);
        let y = y_proportion * (19.0 * tile_width);
        let platform_l = Platform {
            collider: world_l.add_solid(vec2(x, y), 4 * tile_width as i32, tile_width as i32),
            speed: 50.,
        };

        world = Some(world_l);
        player = Some(player_l);
        platform = Some(platform_l);
        tiled_map = Some(tiled_map_l);
    }
    loop {
        touched = false;
        for touch in touches().iter().take(1) {
            match touch.phase {
                TouchPhase::Ended => {
                    touched = true;
                }
                _ => (),
            };
        }
        clear_background(BLACK);

        if r.is_ok() {
            let world = world.as_mut().unwrap();
            let mut platform = platform.as_mut().unwrap();
            let mut player = player.as_mut().unwrap();
            let tiled_map = tiled_map.as_ref().unwrap();
            let tile_width = screen_width() / main_layer_width as f32;

            tiled_map.draw_tiles(
                "main layer",
                Rect::new(
                    0.0,
                    0.0,
                    tile_width * main_layer_width as f32,
                    19.0 * tile_width,
                ),
                None,
            );

            // draw platform
            {
                let pos = world.solid_pos(platform.collider);
                let src_tile_width = 8.0;
                tiled_map.spr_ex(
                    "tileset",
                    Rect::new(
                        6.0 * src_tile_width,
                        0.0,
                        4.0 * src_tile_width,
                        src_tile_width,
                    ),
                    Rect::new(pos.x, pos.y, 4.0 * tile_width, tile_width),
                )
            }

            // draw player
            {
                // sprite id from tiled
                const PLAYER_SPRITE: u32 = 120;

                let pos = world.actor_pos(player.collider);
                if player.speed.x >= 0.0 {
                    tiled_map.spr(
                        "tileset",
                        PLAYER_SPRITE,
                        Rect::new(pos.x, pos.y, tile_width, tile_width),
                    );
                } else {
                    tiled_map.spr(
                        "tileset",
                        PLAYER_SPRITE,
                        Rect::new(pos.x + tile_width, pos.y, -tile_width, tile_width),
                    );
                }
            }

            // player movement control
            {
                let pos = world.actor_pos(player.collider);
                let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));
                // draw_rectangle_lines(
                //     pos.x,
                //     pos.y,
                //     tile_width,
                //     tile_width,
                //     2.0,
                //     if on_ground { GREEN } else { RED },
                // );
                if on_ground == false {
                    player.speed.y += 500. * get_frame_time();
                }

                if right_touch {
                    player.speed.x = 100.0;
                } else if left_touch {
                    player.speed.x = -100.0;
                } else {
                    player.speed.x = 0.;
                }

                if up_touch {
                    if on_ground {
                        let proportion = 120.0 / 154.0;
                        let speed = proportion * (19.0 * tile_width);
                        player.speed.y = -speed;
                    }
                }

                up_touch = false;
                down_touch = false;
                left_touch = false;
                right_touch = false;

                world.move_h(player.collider, player.speed.x * get_frame_time());
                world.move_v(player.collider, player.speed.y * get_frame_time());
            }

            // platform movement
            {
                world.solid_move(platform.collider, platform.speed * get_frame_time(), 0.0);
                let pos = world.solid_pos(platform.collider);
                let min_x_proportion = 150. / 320.;
                let min_x = min_x_proportion * (tile_width * main_layer_width as f32);
                let max_x_proportion = 220. / 320.;
                let max_x = max_x_proportion * (tile_width * main_layer_width as f32);
                if platform.speed > 1. && pos.x >= max_x {
                    platform.speed *= -1.;
                }
                if platform.speed < -1. && pos.x <= min_x {
                    platform.speed *= -1.;
                }
            }
            // for (x, y, tile) in tiled_map.tiles("main layer", None) {
            //     if tile.is_some() {
            //         draw_rectangle_lines(
            //             x as f32 * tile_width,
            //             y as f32 * tile_width,
            //             tile_width,
            //             tile_width,
            //             2.0,
            //             YELLOW,
            //         );
            //     }
            // }
        }

        draw_text(format!("TEXT0: {}", text0).as_str(), 10., 30., 32., WHITE);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 10., 60., 32., WHITE);
        draw_text(
            format!("SCREEN: {}x{}", screen_width(), screen_height()).as_str(),
            10.,
            90.,
            32.,
            WHITE,
        );

        for touch in touches().iter().take(1) {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => {
                    touch_start = touch.position;
                    (GREEN, 80.0)
                }
                TouchPhase::Stationary => (RED, 60.0),
                TouchPhase::Moved => (ORANGE, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),

                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_line(
                touch_start.x,
                touch_start.y,
                touch.position.x,
                touch.position.y,
                2.,
                fill_color,
            );
            let line_end = Vec2::new(touch_start.x, touch_start.y - size);
            for i in 0..4 {
                let new_end = rotate(line_end, touch_start, 90.0 * i as f32 + 45.0);
                draw_line(
                    touch_start.x,
                    touch_start.y,
                    new_end.x,
                    new_end.y,
                    2.,
                    fill_color,
                );
            }
            let angle = (touch_start.y - touch.position.y)
                .atan2(touch_start.x - touch.position.x)
                .to_degrees();
            let seg_ang;
            if touch_start != touch.position {
                match angle {
                    x if x >= 0. && x < 45. => {
                        left_touch = true;
                        seg_ang = 135.;
                    }
                    x if x >= 45. && x < 135. => {
                        up_touch = true;
                        seg_ang = 225.;
                    }
                    x if x >= 135. && x <= 180. => {
                        right_touch = true;
                        seg_ang = 315.;
                    }
                    x if x <= -135. && x >= -180. => {
                        right_touch = true;
                        seg_ang = 315.;
                    }
                    x if x <= -45. && x > -135. => {
                        down_touch = true;
                        seg_ang = 45.;
                    }
                    x if x < 0. && x > -45. => {
                        left_touch = true;
                        seg_ang = 135.;
                    }
                    _ => panic!("Wrong angle! How did you even make this?!"),
                }
                draw_segment(touch_start.x, touch_start.y, size, seg_ang, SKYBLUE);
            }
            draw_circle_lines(touch_start.x, touch_start.y, size, 2., fill_color);
            draw_circle(touch.position.x, touch.position.y, size, fill_color);
            draw_text(
                format!("ANGLE: {}", angle).as_str(),
                10.,
                30.,
                20.,
                DARKGRAY,
            );
        }
        next_frame().await
    }
}
