extern crate dogeylvania;
extern crate rand;
extern crate tcod;

use dogeylvania::actors::*;
use dogeylvania::dogemaths::*;
use dogeylvania::generator;
use dogeylvania::maps::*;
use dogeylvania::screens::Screen;
use dogeylvania::skills::{Skill, SkillTypes};
use dogeylvania::tiles::*;
use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::{FovAlgorithm, Map as FovMap};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

#[derive(PartialEq, Debug)]
enum Actions {
    ActionTook,
    No,
    Exit,
}

fn keys(key: Key, screen: &mut Screen, actors: &mut [Actor], map: &mut Map) -> Actions {
    use tcod::input::KeyCode::*;
    use Actions::*;
    use SkillTypes::*;
    match key {
        Key { code: Escape, .. } => Exit,
        Key { code: Up, .. } | Key { code: NumPad8, .. } => {
            match Skill::use_skill(move_attack, 0, Direction::NORTH, 1, map, actors, screen) {
                true => ActionTook,
                false => No,
            }
        }
        Key { code: Down, .. } | Key { code: NumPad2, .. } => {
            match Skill::use_skill(move_attack, 0, Direction::SOUTH, 1, map, actors, screen) {
                true => ActionTook,
                false => No,
            }
        }
        Key { code: Left, .. } | Key { code: NumPad4, .. } => {
            match Skill::use_skill(move_attack, 0, Direction::WEST, 1, map, actors, screen) {
                true => ActionTook,
                false => No,
            }
        }
        Key { code: Right, .. } | Key { code: NumPad6, .. } => {
            match Skill::use_skill(move_attack, 0, Direction::EAST, 1, map, actors, screen) {
                true => ActionTook,
                false => No,
            }
        }
        Key { code: NumPad9, .. } => {
            match Skill::use_skill(move_attack, 0, Direction::NORTHEAST, 1, map, actors, screen) {
                true => ActionTook,
                false => No,
            }
        }
        Key { code: NumPad7, .. } => {
            match Skill::use_skill(move_attack, 0, Direction::NORTHWEST, 1, map, actors, screen) {
                true => ActionTook,
                false => No,
            }
        }
        Key { code: NumPad3, .. } => {
            match Skill::use_skill(move_attack, 0, Direction::SOUTHEAST, 1, map, actors, screen) {
                true => ActionTook,
                false => No,
            }
        }
        Key { code: NumPad1, .. } => {
            match Skill::use_skill(move_attack, 0, Direction::SOUTHWEST, 1, map, actors, screen) {
                true => ActionTook,
                false => No,
            }
        }

        _ => No,
    }
}

fn draw(screen: &mut Screen, actors: &mut [Actor], map: &mut Map) {
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
    for actor in actors {
        actor.draw(screen);
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
    let mut root = Root::initializer()
        .font("Resources/terminal12x12_gs_ro.png", FontLayout::AsciiInRow)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dogeylvania")
        .init();
    let mut map = Map::new_default(
        SCREEN_WIDTH as usize,
        SCREEN_HEIGHT as usize - 10,
        Tile::gold(),
    );
    generator::generate(&mut map);
    let mut screen = Screen {
        root: root,
        con: Offscreen::new(map.width() as i32, map.height() as i32),
        mouse: Default::default(),
    };
    let mut key = Default::default();
    tcod::system::set_fps(20);

    let mut actors = vec![];
    let mut player = Actor::new(
        5,
        5,
        1 as char,
        colors::DARK_SKY,
        "Doge".to_string(),
        SkillTypes::hit,
    );
    player.skills.push(Skill::move_attack());
    actors.push(player);

    while !screen.root.window_closed() {
        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => screen.mouse = m,
            Some((_, Event::Key(k))) => key = k,
            _ => key = Default::default(),
        }
        draw(&mut screen, &mut actors, &mut map);
        screen.root.flush();
        for actor in &actors {
            actor.clear(&mut screen);
        }
        let action = keys(key, &mut screen, &mut actors, &mut map);
        if action == Actions::Exit {
            break;
        }

        //Next turn
        if action == Actions::ActionTook {
            for a in 0..actors.len() {
                for s in 0..actors[a].skills.len() {
                    actors[a].skills[s].cool_down_left -= 1;
                }
            }
        }
    }
}
