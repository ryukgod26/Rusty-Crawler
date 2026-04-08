use specs::prelude::*;
use super::{Viewshed,Monster,Name,Mqp,Position};
use rltk::{Point,console};

pub struct MonsterAI{}

impl <'a> System<'a> for MonsterAI{
    #[allow(clippy::time_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData){
        let (mut map,player_pos,mut viewshed,monster,name, mut position) = data;
    gs.ecs.insert(map);
    
    gs.ecs.insert(Point::new{player_x,player_y});

        for (mut viewshed,_monster,name,mut pos) in (&mut viewshed,&monster, name, &mut player_pos).join(){
            if viewshed.visible_tiles.contains(&*player_pos){
                console::log(&format!("{} Monster Speaks",name.name));
                let path = rltk::a_star_search(
                    map.xy_index(pos.x,pos.y) as i32,
                    map.xy_index(player_pos.x,player_pos.y) as i32,
                    &mut *map
                    );
                if path.success && path.steps.len()>1{
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
//            console::log("Monster Exists");
        }
    }
}
