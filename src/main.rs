#![allow(non_snake_case)]
extern crate dogeylvania;
extern crate rand;
extern crate tcod;

use dogeylvania::actors::*;
use dogeylvania::dogemaths::*;
use dogeylvania::dogestuff::{Actions, Screen, Ui, MSG};
use dogeylvania::generator;
use dogeylvania::maps::*;
use dogeylvania::skills::{Skill, SkillTypes};
use tcod::colors;
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 60;
const FOV_RADIUS: i32 = 10;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

fn keys(screen: &mut Screen, actors: &mut Vec<Actor>, map: &mut Map) -> Actions {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use Actions::*;
    use SkillTypes::*;

    let key = screen.root.wait_for_keypress(true);

    let dir = match (key, actors[0].alive) {
        (Key { code: Escape, .. }, _) => return Exit,
        (Key { code: Shift, .. }, _) => {
            screen.fov_enable = !screen.fov_enable;
            return NoAction;
        }
        (Key { code: Up, .. }, true) | (Key { code: NumPad8, .. }, true) => Some(Direction::NORTH),
        (Key { code: Down, .. }, true) | (Key { code: NumPad2, .. }, true) => {
            Some(Direction::SOUTH)
        }
        (Key { code: Left, .. }, true) | (Key { code: NumPad4, .. }, true) => Some(Direction::WEST),
        (Key { code: Right, .. }, true) | (Key { code: NumPad6, .. }, true) => {
            Some(Direction::EAST)
        }
        (Key { code: NumPad9, .. }, true) => Some(Direction::NORTHEAST),
        (Key { code: NumPad7, .. }, true) => Some(Direction::NORTHWEST),
        (Key { code: NumPad3, .. }, true) => Some(Direction::SOUTHEAST),
        (Key { code: NumPad1, .. }, true) => Some(Direction::SOUTHWEST),
        (Key { code: NumPad5, .. }, true) => return TookAction,
        (Key { printable: 'r', .. }, _) => {
            new_map(screen, actors, map, true);
            return NoAction;
        }
        (Key { printable: 'n', .. }, _) => {
            new_map(screen, actors, map, false);
            return NoAction;
        }
        _ => None,
    };
    match dir {
        Some(dir) => Skill::use_skill(move_attack, 0, None, dir, 1, map, actors, screen),
        None => NoAction,
    }
}

fn new_map(screen: &mut Screen, actors: &mut Vec<Actor>, map: &mut Map, restart: bool) {
    map.re_default();
    generator::generate(map);
    let openSpace = generator::find_open_space(map);
    if restart {
        actors.truncate(0);
        let mut player = Actor::new(
            openSpace.0 as i32,
            openSpace.1 as i32,
            2 as char,
            colors::DARK_SKY,
            "Doge".to_string(),
            true,
            true,
        );
        player.stats = Some(Stats {
            hp: 20,
            max_hp: 20,
            atk: 10,
            def: 3,
            xp: 0,
            on_death: DeathCallBack::Player,
        });
        player.skills.push(Skill::move_attack());
        player.skills.push(Skill::hit());
        actors.push(player);
    } else {
        actors.drain(1..actors.len());
        actors[0].x = openSpace.0 as i32;
        actors[0].y = openSpace.1 as i32;
        if let Some(stats) = &mut actors[0].stats {
            stats.hp = stats.max_hp;
        }
        map.set_floor(map.get_floor() + 1);
    }

    use dogeylvania::ais::Ai;
    for _ in 0..5 {
        let emptyPos = generator::find_open_space_from(map, openSpace.0, openSpace.1, 5.0);
        let mut spider = Actor::new(
            emptyPos.0 as i32,
            emptyPos.1 as i32,
            'X',
            colors::RED,
            "Tiny Spider".to_string(),
            true,
            true,
        );
        spider.skills.push(Skill::move_attack());
        spider.skills.push(Skill::hit());
        spider.ai = Some(Ai);
        let mult = 2 * map.get_floor();
        spider.stats = Some(Stats {
            hp: 10 + (rand::random::<f32>() * mult as f32).ceil() as i32,
            max_hp: 10 + (rand::random::<f32>() * mult as f32).ceil() as i32,
            atk: 4 + (rand::random::<f32>() * mult as f32).ceil() as i32,
            def: 1 + (rand::random::<f32>() * mult as f32).ceil() as i32,
            xp: 0,
            on_death: DeathCallBack::Monster,
        });
        actors.push(spider);
    }
    for y in 0..map.height() {
        for x in 0..map.width() {
            screen.fov_map.set(
                x as i32,
                y as i32,
                !map.get(x, y).block_light,
                !map.get(x, y).block_move,
            );
        }
    }
    screen.con.clear();
    if restart {
        map.set_floor(1);
        let color = colors::LIGHTER_SEPIA;
        screen
            .messages
            .add_message(format!("Welcome to Dogeylvania!"), colors::LIGHTER_LIME);
        screen
            .messages
            .add_message(format!("Your task is to climb the abyss"), color);
        screen.messages.add_message(
            format!("Find the blue door to proceed to next floor"),
            color,
        );
        screen.messages.add_message(
            format!("When you enter a new floor you will heal, but..."),
            color,
        );
        screen
            .messages
            .add_message(format!("Each floor the enemies will get stronger!"), color);
    } else {
        screen.messages.add_message(
            format!("You reached floor {}", map.get_floor()),
            colors::LIGHTER_FUCHSIA,
        );
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

fn draw_ui(ui: &mut Ui, screen: &mut Screen, actors: &[Actor]) {
    if let Some(stats) = actors[0].stats.as_ref() {
        ui.ui.set_default_foreground(colors::DARK_SKY);
        ui.ui.print(
            1,
            1,
            format!(
                "{} HP: {}, ATK: {}, DEF: {}",
                actors[0].name, stats.hp, stats.atk, stats.def
            ),
        );
    }
    let height = ui.ui.height();
    blit(
        &mut ui.ui,
        (0, 0),
        (SCREEN_WIDTH, height),
        &mut screen.root,
        (0, SCREEN_HEIGHT - height),
        1.,
        1.,
    );

    ui.msg.clear();

    for y in 0..screen.messages.msg.len() {
        let msg = &screen.messages.msg[y];
        ui.msg.set_default_foreground(msg.1);
        ui.msg.print(1, 16 - (1 + y) as i32, &msg.0);
    }

    let height = ui.msg.height();
    blit(
        &mut ui.msg,
        (0, 0),
        (SCREEN_WIDTH, height),
        &mut screen.root,
        (0, SCREEN_HEIGHT - height),
        1.,
        1.,
    );
}

fn main() {
    use dogeylvania::ais::Ai;
    let /*mut*/ root = Root::initializer()
        .font("Resources/terminal12x12_gs_ro.png", FontLayout::AsciiInRow)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dogeylvania")
        .init();
    tcod::system::set_fps(20);
    let mut actors = Vec::new();
    let mut map = Map::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize - 20);
    let mut prev_pos = (-1, -1);
    let mut fov_map = FovMap::new(map.width() as i32, map.height() as i32);
    let mut ui = Ui {
        ui: Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT - map.height() as i32),
        msg: Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT - map.height() as i32 - 3),
    };
    let mut screen = Screen {
        root: root,
        con: Offscreen::new(map.width() as i32, map.height() as i32),
        fov_map: fov_map,
        fov_enable: true,
        last_fov: true,
        messages: MSG { msg: Vec::new() },
    };

    new_map(&mut screen, &mut actors, &mut map, true);

    while !screen.root.window_closed() {
        let fov_recompute = prev_pos != (actors[0].x, actors[0].y);
        draw(&mut screen, &mut actors, &mut map, fov_recompute);
        draw_ui(&mut ui, &mut screen, &actors);
        screen.root.flush();
        for actor in &actors {
            actor.clear(&mut screen);
        }
        prev_pos = (actors[0].x, actors[0].y);
        let action = keys(&mut screen, &mut actors, &mut map);
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
        use dogeylvania::tiles::Tile;
        if map
            .get(actors[0].x as usize, actors[0].y as usize)
            .exit
            .is_some()
        {
            new_map(&mut screen, &mut actors, &mut map, false);
        }
    }
}
