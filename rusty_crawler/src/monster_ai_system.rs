use specs::prelude::*;
use crate::RunState;

use super::{Viewshed,Monster,Name,Map,Position};
use rltk::{Point,console};

pub struct MonsterAI{}

impl <'a> System<'a> for MonsterAI{
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData){
        let (mut map,player_pos,player_entity,runstate,entities,mut viewshed,monster,name, mut position) = data;

        if *runstate != RunState::MonsterTurn { return; }
    gs.ecs.insert(map);
    
    gs.ecs.insert(Point::new{player_x,player_y});

        for (mut viewshed,_monster,name,mut pos) in (&mut viewshed,&monster, name, &mut player_pos).join(){
            if viewshed.visible_tiles.contains(&*player_pos){
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x,pos.y),player_pos);
                if distance < 1.5{
                console::log(&format!("{} Monster Speaks",name.name));
                return;
                }
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
