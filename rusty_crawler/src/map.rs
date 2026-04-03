use rltk::{RGB,Rltk};
use super::{Rect};
use std::cmp::{min,max};
use rltk::{RandomNumberGenerator,Algorithm2D,BaseMap};

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
    pub visible_tiles: Vec<bool>
}

impl Algorithm2D for Map{
    fn dimensions(&self) -> Point{
        Point::new(self.width,self.heigbt)
    }
}

impl BaseMap for Map{
    fn is_opaque(&self, idx: usize) -> bool{
        self.tiles[idx as usize] == TileType::Wall
    }
}

impl Map{

pub fn xy_index(&self,x: i32,y:i i32) -> usize{
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
        if index > 0 && index < 80*50{
//            map[index as usize] = TileType::Floor;
            self.tiles[index as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(&mut self,y1: i32,y2: i32,x: i32) {
    for y in min(y1,y2)..=max(y1,y2){
        let index = self.xy_index(x,y);
        if index > 0 && index < 80 * 50{
//            map[index as usize] = TileType::Floor;
            self.tiles[index as usize] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_and_corridors() -> Map{
    let mut map = Map{
        tiles: vec![TileType::Wall;80*50],
        rooms: Vec::new(),
        width: 80,
        height: 50,
        revealed_tiles: vec![false;80*50],
        visible_tiles: vec![false; 80*50]
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

        for other_room in rooms.iter(){
            if new_room.intersect(other_room) {ok = false}
        }

        if ok{
            map.apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty(){
                let (new_x,new_y) = new_room.center();
                //let (prev_x,prev_y) = rooms[rooms.len() - 1].center();
                let (prev_x,prev_y) =  map.rooms[map.rooms.len() - 1].center();
                if rng.range(0,2) == 1{
                    Self::apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    Self::apply_vertical_tunnel(prev_y, new_y, new_x);
                } else{
                    Self::apply_vertical_tunnel(prev_y, new_y, prev_x);
                    Self::apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }
            map.rooms.push(new_room);
        }

    }
    (rooms,map)
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
                    TileType::Floor{
//                        ctx.set(x,y,RGB::from_f32(.5,.5,.5),RGB::from_f32(0.,0.,0.),rltk::to_cp437('.'));


                        glyph = rltk::to_cp437('.');
                        fg = RGB::from_f32(0.,.5,.5);
                    }
                    TileType::Wall{
  //                      ctx.set(x,y,RGB::from_f32(0.,1.,0.),RGB::from_f32(0.,0.,0.),rltk::to_cp437('#'));
                    
                        glyph = rltk::to_cp436('#');
                        fg = RGB::from_f32(0.,1.,0.);
                    }
                }
                if !map.visible_tiles[idx] {fg = fg.to_grayscale() }
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


pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor;80*50];

    for x in 0..80{
        map[Self::xy_index(x,0)] = TileType::Wall;
        map[Self::xy_index(x,49)] = TileType::Wall;
    }

    for y in 0..50{
        map[Self::xy_index(0,y)] = TileType::Wall;
        map[Self::xy_index(79,y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400{
        let x = rng.roll_dice(1,79);
        let y = rng.roll_dice(1,49);
        let idx = Self::xy_index(x,y);

        if idx != Self::xy_index(40,25){
            map[idx] = TileType::Wall;
        }
    }
    map
    
}
