use specs::prelude::*;
use super::{Viewshed,Monster,Name};
use rltk::{Point,console};

pub struct MonsterAI{}

impl <'a> System<'a> for MonsterAI{
    type SystemData = ( ReadStorage<'a, Point>,
                        ReadStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData){
        let (player_pos,viewshed,monster,name) = data;

        for (viewshed,_monster) in (&viewshed,&monster).join(){
            if viewshed.visible_tiles.contains(&*player_pos){
                console::log(&format!("Monster Speaks",name.name));
            }
//            console::log("Monster Exists");
        }
    }
}
