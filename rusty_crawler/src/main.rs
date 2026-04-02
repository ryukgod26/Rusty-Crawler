mod components;
mod map;
mod player;
mod rect;
pub use components::*;
pub use map::*;
pub use player::*;
pub use rect::*;
use rltk::{Rltk,GameState,RGB};
use specs::prelude::*;



pub struct State{
    ecs: World
}

struct LeftWalker{}

impl GameState for State{
    fn tick(&mut self,ctx: &mut Rltk){

        ctx.cls();

        self.run_systems();
        player_input(self,ctx);
        self.ecs.maintain();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map,ctx);

        for (position, render) in (&positions, &renderables).join() {
            ctx.set(position.x,position.y,render.fg,render.bg,render.glyph);
        }
      //  ctx.print(1,1,"Hi");
    }
}

impl<'a> System<'a> for LeftWalker{
    type SystemData = (ReadStorage<'a, LeftMover>,
                        WriteStorage<'a, Position>);

    fn run(&mut self,(lefty,mut pos): Self::SystemData){
        for (_lefty,pos) in (&lefty,&mut pos).join(){
            pos.x -= 1;
            if pos.x < 0 {pos.x = 79;}
        }
    }
}

impl State{
    fn run_systems(&mut self){
//        let mut lw = LeftWalker{};
//        lw.run_now(&self.ecs);
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError{
    let mut gs = State{
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gc.ecs.register::<Viewshed>();


    let (rooms,map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let(player_x,player_y) = rooms[0].center();


    gs.ecs.create_entity()
        .with(Position{x:player_x, y: player_y})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{visible_tiles: Vec::new(),range: 8})
        .build();

    // for i in 1..=10{
    //     gs.ecs.create_entity()
    //         .with(Position{x:i *7, y: 20})
    //         .with(Renderable{
    //             glyph: rltk::to_cp437('#'),
    //             fg: RGB::named(rltk::RED),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //     .build();
    // }
    gs.ecs.insert(new_map());
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
