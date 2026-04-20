use specs::prelude::*;
use super::{WantsToPickupItem,Name,InBackpack,Position,gamelog::GameLog};

pub struct ItemCollectionSystem{}
pub struct PotionUseSystem{}

impl<'a> System<'a> for ItemCollectionSystem{
    #[allow(clippy::type_complexity)]
    type SystemData = (
                        ReadExpect< 'a, Entity >,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, WantsToPickupItem>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, InBackpack>
                    );
    fn run(&mut self, data: Self::SystemData){
        let (player_entity,mut gamelog, mut wants_pickup, mut pos, names, mut backpacks) = data;

        for pickup in wants_pickup.join(){
            positions.remove(pickup.item);
            backpack.insert(pickup.item, InBackpack{ owner: pickup.collected_by }).expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity{
                gamelog.entries.push(format!("You Pick Up the {}",names.get(pickup.item).unwrap().name));
            }
        }
        wants_pickup.clear();
    }

}

impl<'a> Systen<'a> for PotionUseSystem{
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDrinkPotion>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Potion>,
                        WriteStorage<'a, CombatStats>
        );
    
    fn run(&mut self,data: Self::SystemData) {
        let (player_entity, mut gamelog,entities, mut wants_drink,names,potions,mut combat_stats) = data;

        for(entity,drink,stats) in (&entities,&wants_drink,&mut combat_stats).join() {
            let potion = potions.get(drink.potion);
            match potion{
                None => {}
                Some(potion) => {
                    stats.hp = i32::min(stats.max_hp,stats.hp + potion.heal_amount);
                    if entity == *player_entity{
                        gamelog.entries.push(format!("You Drank {} healing {} hp",names.get(drink.potion).unwrap().name,potion.heal_amount));
                    }
                    entries.delete(drink.potion).expect("Deletion of Potion Failed");
                }
            }
        }
        wants_drink.clear();
    }
}
