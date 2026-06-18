use specs::prelude::*;
use crate::{CombatStats, Potion, WantsToDrinkPotion, WantsToDropItem};

use super::{WantsToPickupItem,Name,InBackpack,Position,gamelog::GameLog};

pub struct ItemCollectionSystem{}
pub struct ItemUseSystem{}
pub struct ItemDropSystem{}

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
            pos.remove(pickup.item);
            backpacks.insert(pickup.item, InBackpack{ owner: pickup.collected_by }).expect("Unable to insert backpack entry");

            if pickup.collected_by == *player_entity{
                gamelog.entries.push(format!("You Pick Up the {}",names.get(pickup.item).unwrap().name));
            }
        }
        wants_pickup.clear();
    }

}

impl<'a> System<'a> for ItemDropSystem{
    #[allow(clippy::type_complexity)]
    type SystemData = (
                        ReadExpect<'a, Entity>,
                        WriteExpect<'a,GameLog>,
                        Entities<'a>,
                        WriteStorage<'a,WantsToDropItem>,
                        ReadStorage<'a,Name>,
                        WriteStorage<'a,Position>,
                        WriteStorage<'a,InBackpack>
        );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, entities, mut wants_drop, names, mut positions, mut backpack) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos: Position = Position {x: 0,y: 0};
            {
                let entity_pos = positions.get(entity).unwrap();
                dropper_pos.x = entity_pos.x;
                dropper_pos.y = entity_pos.y;
            }
            positions.insert(to_drop.item, Position{x: dropper_pos.x,y: dropper_pos.y}).except("Unable to Insert Position");
            backpack.remove(to_drop.item);

            if entity == *player_entity{
                gamelog.entries.push(format!("You droopped the {}", names.get(to_drop.item).unwrap().name));

            }
        }
        wants_drop.clear();
    }
}

impl<'a> System<'a> for ItemUseSystem{
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
//            let potion = potions.get(drink.potion);

                let item_heals = healing.get(useitem.item);

            match potion{
                None => {}
                Some(healer) => {
                    stats.hp = i32::min(stats.max_hp,stats.hp + potion.heal_amount);
                    if entity == *player_entity{
                        gamelog.entries.push(format!("You Drank {} healing {} hp",names.get(drink.potion).unwrap().name,potion.heal_amount));
                    }
                    entities.delete(drink.potion).expect("Deletion of Potion Failed");
                }
            }

            let consumable = consumables.get(usitem.item);
            match consumable{
                None => {}
                Some(_) => {
                    entities.delete(useitem.item).except("Delete Failed");

                }
            }
        }
        wants_drink.clear();

        let item_damages = inflict_damage.get(useitem.item);
        match item_damages {
            None =>{}
            Some(damage) => {
                let target_point = useitem.target.unwrap();
                let idx = map.xy_idx(target_point.x, target_point.y);
                used_item = false;

                for mob in map.tile_content[idx].iter() {
                    SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                    if entity == *player_entity{
                        let mob_name = names.get(*mob).unwrap();
                        let item_name = names.get(useitem.item).unwrap();
                        gamelog.entries.push(format!("You used {} on {} damaging {} hp.", item_name.name, mob_name.name, damage.damage));
                    }
                    used_item = true;
                }
            }
        }

    }
}
