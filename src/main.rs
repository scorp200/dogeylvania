#![allow(non_snake_case)]
extern crate dogeylvania;
extern crate rand;
extern crate tcod;

use dogeylvania::actors::*;
use dogeylvania::dogemaths::*;
use dogeylvania::dogestuff::{Actions, Screen};
use dogeylvania::generator;
use dogeylvania::maps::*;
use dogeylvania::skills::{Skill, SkillTypes};
//use dogeylvania::tiles::*;
use tcod::colors::{self/*, Color*/};
use tcod::console::*;
use tcod::input::{self, Event, Key/*, Mouse*/};
use tcod::map::{FovAlgorithm, Map as FovMap};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FOV_RADIUS: i32 = 10;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

fn keys(key: Key, screen: &mut Screen, actors: &mut [Actor], map: &mut Map) -> Actions {
    use tcod::input::KeyCode::*;
    use Actions::*;
    use SkillTypes::*;
    let dir = match key {
        Key { code: Escape, .. } => return Exit,
        Key { code: Shift, .. } => {
            screen.fov_enable = !screen.fov_enable;
            return NoAction;
        }
        Key { code: Up, .. } | Key { code: NumPad8, .. } => Some(Direction::NORTH),
        Key { code: Down, .. } | Key { code: NumPad2, .. } => Some(Direction::SOUTH),
        Key { code: Left, .. } | Key { code: NumPad4, .. } => Some(Direction::WEST),
        Key { code: Right, .. } | Key { code: NumPad6, .. } => Some(Direction::EAST),
        Key { code: NumPad9, .. } => Some(Direction::NORTHEAST),
        Key { code: NumPad7, .. } => Some(Direction::NORTHWEST),
        Key { code: NumPad3, .. } => Some(Direction::SOUTHEAST),
        Key { code: NumPad1, .. } => Some(Direction::SOUTHWEST),
        Key { code: NumPad5, .. } => return TookAction,
        _ => None,
    };
    match dir {
        Some(dir) => Skill::use_skill(move_attack, 0, None, dir, 1, map, actors, screen),
        None => NoAction,
    }
}

fn draw(screen: &mut Screen, actors: &mut [Actor], map: &mut Map, fov_recompute: bool) {
    if fov_recompute {
        screen
            .fov_map
            .compute_fov(actors[0].x, actors[0].y, FOV_RADIUS, true, FOV_ALGO);
    }

    if screen.last_fov != screen.fov_enable {
        screen.con.clear();
    }

    for y in 0..map.height() {
        for x in 0..map.width() {
            let visible = screen.fov_map.is_in_fov(x as i32, y as i32);
            let tile = &mut map.get(x, y);
            if visible {
                tile.explored = true;
            }
            if tile.explored {
                if let Some(bg) = &tile.color.as_ref() {
                    screen.con.set_char_background(
                        x as i32,
                        y as i32,
                        if visible { bg.1 } else { bg.0 },
                        BackgroundFlag::Set,
                    );
                }
                if let Some(chara) = &tile.char.as_ref() {
                    screen
                        .con
                        .set_default_foreground(if visible { chara.2 } else { chara.1 });
                    screen
                        .con
                        .put_char(x as i32, y as i32, chara.0, BackgroundFlag::None);
                }
            }
        }
    }
    screen.last_fov = screen.fov_enable;
    if !screen.fov_enable {
        for y in 0..map.height() {
            for x in 0..map.width() {
                let tile = &map.get(x, y);

                if let Some(bg) = &tile.color.as_ref() {
                    screen
                        .con
                        .set_char_background(x as i32, y as i32, bg.1, BackgroundFlag::Set);
                }
                if let Some(chara) = &tile.char.as_ref() {
                    screen.con.set_default_foreground(chara.1);
                    screen
                        .con
                        .put_char(x as i32, y as i32, chara.0, BackgroundFlag::None);
                }
            }
        }
    }
    for actor in actors {
        if screen.fov_map.is_in_fov(actor.x, actor.y) || !screen.fov_enable {
            actor.draw(screen);
        }
    }
    blit(
        &mut screen.con,
        (0, 0),
        (map.width() as i32, map.height() as i32),
        &mut screen.root,
        (0, 0),
        1.,
        1.,
    );
}

fn main() {
    let /*mut*/ root = Root::initializer()
        .font("Resources/terminal12x12_gs_ro.png", FontLayout::AsciiInRow)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dogeylvania")
        .init();
    let mut map = Map::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize - 10);
    generator::generate(&mut map);
    let mut key = Default::default();
    tcod::system::set_fps(20);

    let mut actors = vec![];
    let openSpace = generator::find_open_space(&mut map);
    let mut player = Actor::new(
        openSpace.0 as i32,
        openSpace.1 as i32,
        2 as char,
        colors::DARK_SKY,
        "Doge".to_string(),
        true,
    );
    let mut prev_pos = (-1, -1);
    player.skills.push(Skill::move_attack());
    player.skills.push(Skill::hit());
    actors.push(player);

	use dogeylvania::ais::Ai;
	for _ in 0..5 {
		let emptyPos = generator::find_open_space_from(&mut map, openSpace.0, openSpace.1, 5.0);
	    let mut spider = Actor::new(emptyPos.0 as i32, emptyPos.1 as i32, 'X', colors::RED, "Tiny Spider".to_string(), true);
	    spider.skills.push(Skill::move_attack());
	    spider.skills.push(Skill::hit());
	    spider.ai = Some(Ai);
	    actors.push(spider);
	}

    let mut fov_map = FovMap::new(map.width() as i32, map.height() as i32);
    for y in 0..map.height() {
        for x in 0..map.width() {
            fov_map.set(
                x as i32,
                y as i32,
                !map.get(x, y).block_light,
                !map.get(x, y).block_move,
            );
        }
    }
    let mut screen = Screen {
        root: root,
        con: Offscreen::new(map.width() as i32, map.height() as i32),
        fov_map: fov_map,
        fov_enable: true,
        last_fov: true,
        mouse: Default::default(),
    };

    while !screen.root.window_closed() {
        let fov_recompute = prev_pos != (actors[0].x, actors[0].y);
        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => screen.mouse = m,
            Some((_, Event::Key(k))) => key = k,
            _ => key = Default::default(),
        }
        draw(&mut screen, &mut actors, &mut map, fov_recompute);
        screen.root.flush();
        for actor in &actors {
            actor.clear(&mut screen);
        }
        prev_pos = (actors[0].x, actors[0].y);
        let action = keys(key, &mut screen, &mut actors, &mut map);
        if action == Actions::Exit {
            break;
        }

        //Next turn
        if action == Actions::TookAction {
            //ai
            for id in 0..actors.len() {
                if actors[id].ai.is_some() {
                    Ai::take_turn(id, &map, &mut actors, &mut screen);
                }
            }

            //reduce cooldown
            for a in 0..actors.len() {
                for s in 0..actors[a].skills.len() {
                    let cdl = actors[a].skills[s].cool_down_left;
                    if cdl > 0 {
                        actors[a].skills[s].cool_down_left = cdl - 1;
                    }
                }
            }
        }
    }
}
