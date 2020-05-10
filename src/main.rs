use rltk::{GameState, BTerm, RltkBuilder, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{min, max};

struct State {
    world: World
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker { };
        lw.run_now(&self.world);
        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (position, renderable) in (&positions, &renderables).join()  {
            ctx.set(position.x, position.y, renderable.fg, renderable.bg, renderable.glyph)
        }
    }
}

struct Player { }

impl Component for Player {
    type Storage = VecStorage<Self>;
}

struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct LeftMover { }

impl Component for LeftMover {
    type Storage = VecStorage<Self>;
}

struct LeftWalker { }

impl <'a> System<'a> for LeftWalker {
    type SystemData = (
        ReadStorage<'a, LeftMover>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, (left, mut pos): Self::SystemData) {
        for (_, pos) in (&left, &mut pos).join() {
            pos.x -= 1;

            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

struct Renderable {
    glyph: u16,
    fg: RGB,
    bg: RGB,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}

fn main() {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .unwrap();

    let mut state = State {
        world: World::new()
    };

    state.world.register::<LeftMover>();
    state.world.register::<Player>();
    state.world.register::<Position>();
    state.world.register::<Renderable>();

    state.world
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK)
        })
        .with(Player { })
        .build();

    for i in 0..10 {
        state.world
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK)
            })
            .with(LeftMover { })
            .build();
    }

    rltk::main_loop(context, state);
}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();

    for (_, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79 , max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(state: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        Some(VirtualKeyCode::Left) => try_move_player(-1, 0, &mut state.world),
        Some(VirtualKeyCode::Right) => try_move_player(1, 0, &mut state.world),
        Some(VirtualKeyCode::Up) => try_move_player(0, -1, &mut state.world),
        Some(VirtualKeyCode::Down) => try_move_player(0, 1, &mut state.world),
        _ => ()
    }
}
