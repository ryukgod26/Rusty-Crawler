use wasm_bindgen::prelude::*;
use rltk::{Rltk,GameState};
use specs::prelude::*;
use std::cmp::{max,min}; 
use specs_derive::Component;

struct State{}

impl GameState for State{
    fn tick(&mut self,ctx: &mut Rltk){
        ctx.cls();
        ctx.print(1,1,"Test");
    }
}

fn main() -> rltk::BError{
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rusty Crawler")
        .build()?;
    let gs = State{};
    rltk::main_loop(context,gs)
}

/*
#[wasm_bindgen]
pub fn start() {
    
}
*/
