use rltk::{RGB,Rltk};
use super::{Rect};
use std::cmp::{min,max};
use rltk::{RandomNumberGenerator,Algorithm2D,BaseMap,Point};
use specs::prelude::*;

const MAPWIDTH: usize = 80;
const MAPHEIGHT: usize = 50;
const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq,Copy,Clone)]
pub enum TileType{
    Wall,Floor
}

pub struct Map{
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>
}

impl Algorithm2D for Map{
    fn dimensions(&self) -> Point{
        Point::new(self.width,self.height)
    }
}

impl BaseMap for Map{
    fn is_opaque(&self, idx: usize) -> bool{
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_pathing_distance(&self,idx1: usize,idx2: usize) -> f32{
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1,p2)
    }
}

impl Map{

pub fn xy_index(&self,x: i32,y:i32) -> usize{
    (y as usize * 80) + x as usize
}



fn apply_room_to_map(&mut self,room: &Rect) {
    for y in room.y1 +1..=room.y2{
        for x in room.x1+1..=room.x2{
            let idx = self.xy_index(x,y);
            self.tiles[idx] = TileType::Floor;
            //map[Self::xy_index(x,y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(&mut self,x1: i32, x2: i32, y: i32) {
    for x in min(x1,x2)..=max(x1,x2){
        let index = self.xy_index(x,y);
        if index > 0 && index < MAPCOUNT{
//            map[index as usize] = TileType::Floor;
            self.tiles[index as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(&mut self,y1: i32,y2: i32,x: i32) {
    for y in min(y1,y2)..=max(y1,y2){
        let index = self.xy_index(x,y);
        if index > 0 && index < MAPCOUNT{
//            map[index as usize] = TileType::Floor;
            self.tiles[index as usize] = TileType::Floor;
        }
    }
}

fn get_available_exits(&self,idx: usize) -> rltk::SmallVec<[(usize,f32); 10]>{
    let mut exits = rltk::SmallVec::new();
    let x = idx as i32 % self.width;
    let y = idx as i32 / self.width;
    let w = self.width as usize;

    if self.is_exit_valid(x-1,y) {exits.push((idx-1,1.0))}
    if self.is_exit_valid(x+1,y) {exits.push((idx+1,1.0))}
    if self.is_exit_valid(x,y-1) {exits.push((idx-w,1.0))}
    if self.is_exit_valid(x,y+1) {exits.push((idx+w,1.0))}

    // For Diagonal Movement
    if self.is_exit_valid(x-1,y-1) {exits.push(((idx-w)-1, 1.45))}
    if self.is_exit_valid(x+1,y-1) {exits.push(((idx-w)+1, 1.45))}
    if self.is_exit_valid(x-1,y+1) {exits.push(((idx+w)-1, 1.45))}
    if self.is_exit_valid(x+1,y+1) {exits.push(((idx+w)+1, 1.45))}


    exits
}

fn is_exit_valid(&self,x: i32,y: i32) -> bool{
    if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 {return false;}
    let idx = self.xy_index(x,y);
    !self.blocked[idx]
}


pub fn populate_blocked(&mut self) {
    for (i,tile) in self.tiles.iter_mut().enumerate(){
        self.blocked[i] = *tile == TileType::Wall;
    }
}

pub fn clear_content_index(&mut self){
    for content in self.tile_content.iter_mut(){
        content.clear();
    }
}

pub fn new_map_rooms_and_corridors() -> Map{
    let mut map = Map{
        tiles: vec![TileType::Wall;MAPCOUNT],
        rooms: Vec::new(),
        width: 80,
        height: 50,
        revealed_tiles: vec![false;MAPCOUNT],
        visible_tiles: vec![false; MAPCOUNT],
        blocked: vec![false;MAPCOUNT],
        tile_content: vec![Vec::new(); MAPCOUNT]
    };
    //let mut map = vec![TileType::Wall;80*50];

//    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for i in 0..MAX_ROOMS{
        let w = rng.range(MIN_SIZE,MAX_SIZE);
        let h = rng.range(MIN_SIZE,MAX_SIZE);
        let x = rng.roll_dice(1, map.width - w - 1) -1;
        let y = rng.roll_dice(1,map.height - h - 1) -1;
        let new_room = Rect::new(x,y,w,h);
        let mut ok = true;

        for other_room in map.rooms.iter(){
            if new_room.intersect(other_room) {ok = false}
        }

        if ok{
            map.apply_room_to_map(&new_room);

            if !map.rooms.is_empty(){
                let (new_x,new_y) = new_room.center();
                //let (prev_x,prev_y) = rooms[rooms.len() - 1].center();
                let (prev_x,prev_y) =  map.rooms[map.rooms.len() - 1].center();
                if rng.range(0,2) == 1{
                    map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.apply_vertical_tunnel(prev_y, new_y, new_x);
                } else{
                    map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }
            map.rooms.push(new_room);
        }

    }
    map
}
}

/*
pub fn draw_map(map: &[TileType],ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter(){
        match tile{
            TileType::Wall=>{
                ctx.set(x,y,RGB::from_f32(0.0,1.0,0.0),RGB::from_f32(0.0,0.0,0.0),rltk::to_cp437('#'));
            }
            TileType::Floor=>{
                ctx.set(x,y,RGB::from_f32(0.5,0.5,0.5),RGB::from_f32(0.0,0.0,0.0),rltk::to_cp437('.'));
            }
        }
        x += 1;
        if x > 79{
            x = 0;
            y += 1;
        }
    }

}
*/

pub fn draw_map(ecs: &World,ctx: &mut Rltk){
//    let mut viewsheds = ecs.write_storage::<Viewshed>();
//    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

  //  for(_player,viewshed) in (&mut players, &mut viewsheds).join(){
      for(idx,tile) in map.tiles.iter().enumerate(){ 
//        let mut x = 0;
//        let mut y = 0;

  //      for tile in map.tiles.iter(){
      //      let pt = Point::new(x,y);
        //    if viewshed.visible_tiles.contains(&pt) {
              if map.revealed_tiles[idx]{
                let glyph;
                let mut fg;
                match tile{
                    TileType::Floor => {
//                        ctx.set(x,y,RGB::from_f32(.5,.5,.5),RGB::from_f32(0.,0.,0.),rltk::to_cp437('.'));
                        glyph = rltk::to_cp437('.');
                        fg = RGB::from_f32(0.0,0.5,0.5);
                    }
                    TileType::Wall => {
  //                      ctx.set(x,y,RGB::from_f32(0.,1.,0.),RGB::from_f32(0.,0.,0.),rltk::to_cp437('#'));
                    
                        glyph = rltk::to_cp437('#');
                        fg = RGB::from_f32(0.,1.,0.);
                    }
                }
                if !map.visible_tiles[idx] {fg = fg.to_greyscale() }
                ctx.set(x,y,fg,RGB::from_f32(0.,0.,0.),glyph);
            }
    //    }
    x += 1;
    if x > 79{
        x = 0;
        y += 1;
    }
      }
}


// pub fn new_map_test() -> Vec<TileType> {
//     let mut map = vec![TileType::Floor;80*50];

//     for x in 0..80{
//         map[Self::xy_index(x,0)] = TileType::Wall;
//         map[Self::xy_index(x,49)] = TileType::Wall;
//     }

//     for y in 0..50{
//         map[Self::xy_index(0,y)] = TileType::Wall;
//         map[Self::xy_index(79,y)] = TileType::Wall;
//     }

//     let mut rng = rltk::RandomNumberGenerator::new();

//     for _i in 0..400{
//         let x = rng.roll_dice(1,79);
//         let y = rng.roll_dice(1,49);
//         let idx = Self::xy_index(x,y);

//         if idx != Self::xy_index(40,25){
//             map[idx] = TileType::Wall;
//         }
//     }
//     map
    
// }
