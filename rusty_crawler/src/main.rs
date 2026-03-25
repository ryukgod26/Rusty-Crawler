use wasm_bindgen::prelude::*;
use rltk::{Rltk,GameState,RGB,VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max,min}; 
use specs_derive::Component;

struct State{
    ecs: World
}

#[derive(Component)]
struct Position{
    x: i32,
    y: i32
}

#[derive(Component)]
struct Renderable{
    glyph: rltk::FontCharType,
    fg: RGB,
    bg:RGB
}

impl GameState for State{
    fn tick(&mut self,ctx: &mut Rltk){
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        ctx.cls();
        
        for (position, render) in (&positions, &renderables).join() {
            ctx.set(position.x,position.y,render.fg,render.bg,render.glyph);
        }
        ctx.print(1,1,"Hi");
    }
}

fn main() -> rltk::BError{
    let mut gs = State{
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    
    gs.ecs.create_entity()
        .with(Position{x:30, y: 20})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    for i in 1..=10{
        gs.ecs.create_entity()
            .with(Position{x:i *7, y: 20})
            .with(Renderable{
                glyph: rltk::to_cp437('#'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
        .build();
    }
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rusty Crawler")
        .build()?;
//    let gs = State{};
    rltk::main_loop(context,gs)
}

/*
#[wasm_bindgen]
pub fn start() {
    
}
*/
