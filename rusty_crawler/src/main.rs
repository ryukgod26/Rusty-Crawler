mod components;
mod map;
mod player;
mod rect;
mod visibility_system;
mod monster_ai_system;
mod map_indexing_system; 
mod melee_combat_system;
mod damage_system;
mod gui;
mod gamelog;
mod spawner;
pub use components::*;
pub use map::*;
pub use player::*;
pub use rect::*;
pub use monster_ai_system::MonsterAI;
pub use map_indexing_system::*;
pub use melee_combat_system::MeleeCombatSystem;
pub use damage_system::*;
pub use gui::*;
pub use spawner::*;
use visibility_system::VisibilitySystem;
use rltk::{Rltk,GameState,RGB,Point};
use specs::prelude::*;


#[derive(PartialEq,Copy,Clone)]
pub enum RunState{ AwaitingInput, PreRun, PlayerTurn, MonsterTurn }

pub struct State{
    pub ecs: World,
}

struct LeftWalker{}


impl GameState for State{
    fn tick(&mut self,ctx: &mut Rltk){

        ctx.cls();

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate{
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput =>{
                newrunstate = player_input(self,ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn =>{
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        damage_system::delete_the_dead(&mut self.ecs);
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        
        for (pos, render) in (&positions, &renderables).join(){
            let idx = map.xy_index(pos.x, pos.y);
            if map.visible_tiles[idx] { ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph); }
        }

        gui::draw_ui(&self.ecs,ctx);
        // if self.runstate == RunState::Running{
        //     self.run_systems();
        //     damage_systems::delete_the_dead(&mut self.ecs);
        //     self.runstate = RunState::Paused;
        // } else{
        //     self.runstate = player_input(self,ctx);
        // }
/*
        self.run_systems();
        player_input(self,ctx);
        self.ecs.maintain();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        draw_map(&self.ecs,ctx);

        for (position, render) in (&positions, &renderables).join() {
            ctx.set(position.x,position.y,render.fg,render.bg,render.glyph);
        }
*/
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
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem{};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError{
    let mut gs = State{
        ecs: World::new(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();

    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    :q


    let map= Map::new_map_rooms_and_corridors();
    let(player_x,player_y) = map.rooms[0].center();

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rusty Crawler")
        .build()?;

    let mut rng = rltk::RandomNumberGenerator::new();
/*
    for (i,room) in map.rooms.iter().skip(1).enumerate(){
        let (x,y) = room.center();
        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1,2);
        match roll{
            1 => { glyph = rltk::to_cp437('g'); name="Goblin".to_string(); }
            _ => { glyph = rltk::to_cp437('o'); name="Orc".to_string();}
        }
        gs.ecs.create_entity()
            .with(Position{x,y})
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK)
            })
            .with(Viewshed{visible_tiles: Vec::new(),range: 8, dirty: true})
            .with(Monster{})
            .with(Name{name: format!("{} #{}",&name,i)})
            .with(BlocksTile{})
            .with(CombatStats{max_hp: 16,hp: 16, defense: 1,power: 4})
            .build();
        }
*/
    let player_entity = spawner::player(&mut gs.ecs,player_x,player_y);

    for room in map.rooms.iter() {
//        let (x,y) = room.center();
//        spawner::random_monster(&mut gs.ecs,x,y);

        spawner::spawn_room(&mut gs.ecs,room);
    }




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
    // gs.ecs.insert(new_map());
    
//    let gs = State{};
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x,player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome to Rusty Crawler".to_string() ]});
    rltk::main_loop(context,gs)
}

/*
#[wasm_bindgen]
pub fn start() {
    
}
*/
