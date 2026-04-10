use specs::prelude::*;
use super::{CombatStats,SufferDamage};

pub struct DamageSystem{}

impl<'a> System<'a> for DamageSystem{
    type SystemData = (WriteStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for(mut stats,damage) in (&mut stats, &damage).join(){
            stats.hp -= damage.amount.iter().sum::<i32>();
        }
        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World){
        let mut dead: Vec<Entity> = Vec::new();
        //Using this to make the borrow checker happy just got to know about this
        {
            let combat_stats = ecs.read_storage::<CombatStats>();
            let players = ecs.read_storage::<Player>();
            let names = ecs.read_storage::<Name>();
            let mut log = ecs.write_storage::<GameLog>();
            let entities = ecs.entities();

            for(entity,stats) in (&entities,&combat_stats).join() {
                if stats.hp < 1 {
                    let player = player.get(entity);
                    match player{
                    None => {
                        if let Some(victim) = victim {
                            log.entries.push(format!("{} is dead.",&victim.name));
                        }
                        dead.push(entity)
                    }
                    Some(_) => console::log("You are Dead") 

                }
                }
            }
        }

        for victim in dead{
            ecs.delete_entity(victim).expect("Unable to Delete.");
        }
    }
