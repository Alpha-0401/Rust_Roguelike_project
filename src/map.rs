use bracket_lib::prelude::*;
use std::cmp::{max, min};

// Définition des types de terrain
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Stairs,
    Goal,
}

// Fonction pour grille 1D
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

// Fonctions pour generer la carte
fn gen_room(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn gen_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx] = TileType::Floor;
        }
    }
}

fn gen_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx] = TileType::Floor;
        }
    }
}


fn new_map(map_depth: i32) -> (Vec<TileType>, i32, i32, Vec<Point>) {
    let mut map = vec![TileType::Wall; 80 * 50];
    let mut rooms: Vec<Rect> = Vec::new();
    let mut rng = RandomNumberGenerator::new();
    //let mut enemies: Vec<Point> = Vec::new();
    let mut possible_spawns: Vec<Point> = Vec::new();

    let mut player_x = 0;
    let mut player_y = 0;

    for _ in 0..30 {
        let w = rng.range(6, 15);
        let h = rng.range(6, 11);
        let x = rng.range(1, 80 - w - 1);
        let y = rng.range(1, 50 - h - 1);
        
        let new_room = Rect::with_size(x, y, w, h);

        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) { ok = false }
        }

        if ok {
            gen_room(&new_room, &mut map);

            if rooms.is_empty() {
                let center = new_room.center();
                player_x = center.x;
                player_y = center.y;
            } else {
                let new_center = new_room.center();
                let prev_center = rooms[rooms.len()-1].center();
                
                // Enemie au centre des salles
                possible_spawns.push(Point::new(new_center.x, new_center.y));
                
                let new_x = new_center.x;
                let new_y = new_center.y;
                let prev_x = prev_center.x;
                let prev_y = prev_center.y;
                
                if rng.range(0, 2) == 1 {
                    gen_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    gen_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    gen_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    gen_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }
            rooms.push(new_room);
        }
    }
    
    // Choisir 5 points de spawn d'ennemies
    let mut enemies: Vec<Point> = Vec::new();
    for _ in 0..5{
        if possible_spawns.is_empty() {break;}
        let idx = rng.range(0, possible_spawns.len() as i32) as usize;
        enemies.push(possible_spawns.remove(idx));
    }
    
    let last_room_center = rooms[rooms.len() - 1].center();
    let target_idx = xy_idx(last_room_center.x, last_room_center.y);
    
    // Placer les escaliers dans la dernière salle générée (si niveau < 3)
    if map_depth < 3 {
        map[target_idx] = TileType::Stairs;
    } else {
        map[target_idx] = TileType::Goal;
    }

    (map, player_x, player_y, enemies)
}

