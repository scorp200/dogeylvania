extern crate dogeylvania;
extern crate rand;
extern crate tcod;

use dogeylvania::actors::*;
use dogeylvania::dogemaths::*;
use dogeylvania::maps::*;
use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::input::{self, Event, Mouse};
use tcod::map::{FovAlgorithm, Map as FovMap};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

struct Screen {
    root: Root,
    con: Offscreen,
    mouse: Mouse,
}

fn draw(screen: &mut Screen, actors: &mut [Actor], map: &mut Map) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            screen.con.set_default_foreground(map.get_color(x, y));
            screen
                .con
                .put_char(x as i32, y as i32, map.get_char(x, y), BackgroundFlag::None);
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
    let mut root = Root::initializer()
        .font("Resources/terminal12x12_gs_ro.png", FontLayout::AsciiInRow)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Dogeylvania")
        .init();
    let mut map = Map::new_default(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize - 10, 0);
    let mut screen = Screen {
        root: root,
        con: Offscreen::new(map.width() as i32, map.height() as i32),
        mouse: Default::default(),
    };
    let mut key = Default::default();
    tcod::system::set_fps(20);

    let mut actors = vec![];
    let mut player = Actor::new(5, 5, '@', colors::LIGHT_BLUE, "Doge".to_string());
    actors.push(player);

    while !screen.root.window_closed() {
        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => screen.mouse = m,
            Some((_, Event::Key(k))) => key = k,
            _ => key = Default::default(),
        }
        draw(&mut screen, &mut actors, &mut map);
        screen.root.flush();
    }
}
