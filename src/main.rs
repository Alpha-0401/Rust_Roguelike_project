use bracket_lib::prelude::*;
use std::cmp::{max, min};

// Définition des types de terrain
#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
    Stairs,
    Goal,
}

// Fonction pour grille 1D
fn xy_idx(x: i32, y: i32) -> usize {
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

fn new_map(map_depth: i32) -> (Vec<TileType>, i32, i32) {
    let mut map = vec![TileType::Wall; 80 * 50];
    let mut rooms: Vec<Rect> = Vec::new();
    let mut rng = RandomNumberGenerator::new();

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
    
    let last_room_center = rooms[rooms.len() - 1].center();
    let target_idx = xy_idx(last_room_center.x, last_room_center.y);
    
    // Placer les escaliers dans la dernière salle générée (si niveau < 3)
    
    if map_depth < 3 {
        map[target_idx] = TileType::Stairs;
    } else {
        map[target_idx] = TileType::Goal;
    }

    (map, player_x, player_y)
}

//Définition de l'état du jeu (sauvegarde de coordonnées)
struct State {
    player_x: i32,
    player_y: i32,
    map: Vec<TileType>,
    map_depth: i32,
    game_won: bool,
    }

//Boucle principale
impl GameState for State {

    //fonction pour mettre à jour
    fn tick(&mut self, ctx: &mut BTerm) {
        
        //Nettoyer écran à chaque frame
        ctx.cls();
        
        // Si gagné, on affiche l'écran de victoire et on ignore le reste
        if self.game_won {
            ctx.print_color_centered(22, RGB::named(YELLOW), RGB::named(BLACK), "VICTOIRE !");
            ctx.print_color_centered(24, RGB::named(WHITE), RGB::named(BLACK), "Vous avez trouve la sortie du donjon !");
            ctx.print_color_centered(26, RGB::named(GRAY), RGB::named(BLACK), "Appuyez sur Echap (Esc) pour quitter.");
            
            if let Some(VirtualKeyCode::Escape) = ctx.key {
                ctx.quit(); // Ferme le jeu
            }
            return;
        }
        
        let mut new_x = self.player_x;
        let mut new_y = self.player_y;
        
        //Gestion clavier
        if let Some(key) = ctx.key{
        //println!("Tecla presionada: {:?}", key);
            match key{
                VirtualKeyCode::Left => new_x -= 1,
                VirtualKeyCode::Right => new_x += 1,
                VirtualKeyCode::Up => new_y -= 1,
                VirtualKeyCode::Down => new_y += 1,
                
                // Action pour descendre l'escalier ou sortie
                VirtualKeyCode::Space => {
                    let player_idx = xy_idx(self.player_x, self.player_y);
                    if self.map[player_idx] == TileType::Stairs {
                        self.map_depth += 1; // On descend d'un niveau
                        
                        // Générer la nouvelle carte
                        let (new_m, new_px, new_py) = new_map(self.map_depth);
                        
                        // Mettre à jour l'état du jeu
                        self.map = new_m;
                        new_x = new_px;
                        new_y = new_py;
                    } else if self.map[player_idx] == TileType::Goal {
                        // On a atteint sortie
                        self.game_won = true;
                    }
                }
                
                // Ignorer le rest
                _ => {}
            }
            //.clamp(min,max) assure que les limites des valeurs
            new_x = new_x.clamp(0, 79);
            new_y = new_y.clamp(0, 49);
            
            // Collisions
            let destination_idx = xy_idx(new_x, new_y);
            if self.map[destination_idx] != TileType::Wall {
                self.player_x = new_x;
                self.player_y = new_y;
            }
        }
        
        
        
        //Dessiner carte
        for (idx, tile) in self.map.iter().enumerate() {
            let x = idx as i32 % 80;
            let y = idx as i32 / 80;

            match tile {
                TileType::Floor => {
                    ctx.print_color(x, y, RGB::named(DARK_GREEN), RGB::named(BLACK), ".");
                }
                TileType::Wall => {
                    ctx.print_color(x, y, RGB::named(GRAY), RGB::named(BLACK), "#");
                }
                TileType::Stairs => {
                    // Dessiner l'escalier avec le symbole '>'
                    ctx.print_color(x, y, RGB::named(RED), RGB::named(BLACK), ">");
                }
                TileType::Goal => {
                    // Dessiner la meta avec une étoile dorée
                    ctx.print_color(x, y, RGB::named(GOLD), RGB::named(BLACK), "*");
                }
            }
        }
        
        // HUD
        ctx.print_color(1,1,RGB::named(BLUE),RGB::named(BLACK), "Roguelike Test");
        ctx.print_color(1, 2, RGB::named(WHITE), RGB::named(BLACK), &format!("P{}", self.map_depth));
        
        //Generer player
        ctx.print_color(
            self.player_x,
            self.player_y,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            "@"
        );
    }
}

fn main() -> BError {

    //Configuration du contexte
    let context = BTermBuilder::simple80x50()
        .with_title("First Roguelike")
        .build()?;
        
    //Generation map
    let depth = 1;
    let (map_gen, px, py) = new_map(depth);
         
    //Initialiser (Joueur au millieu)
    let gs = State{
        player_x: px,
        player_y: py,
        map: map_gen,
        map_depth: depth,
        game_won: false,
    };
    
    main_loop(context,gs)
}
