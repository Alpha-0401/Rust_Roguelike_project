use bracket_lib::prelude::*;

mod map;
use map::*;
mod game;
use game::*;

fn main() -> BError {

    //Configuration du contexte
    let context = BTermBuilder::simple80x50()
        .with_title("First Roguelike")
        .build()?;
        
    //Generation map
    let depth = 1;
    let (map_gen, px, py, enemies_gen) = new_map(depth);
         
    //Initialiser (Joueur au millieu)
    let gs = State{
        player_x: px,
        player_y: py,
        map: map_gen,
        map_depth: depth,
        game_won: false,
        hp: 3,
        enemies: enemies_gen,
    };
    
    main_loop(context,gs)
}


