use bracket_lib::prelude::*;

//Définition de l'état du jeu (sauvegarde de coordonnées)
struct State {
    player_x: i32,
    player_y: i32,
    }

//Boucle principale
impl GameState for State {
    //fonction pour mettre à jour
    fn tick(&mut self, ctx: &mut BTerm) {
        
        //Lire clavier
        if let Some(key) = ctx.key{
        println!("Tecla presionada: {:?}", key);
            match key{
                // Mouvement vers la gauche (Q/A)
                VirtualKeyCode::Left => self.player_x -= 1,
                // Mouvement vers la droite (D)
                VirtualKeyCode::Right => self.player_x += 1,
                // Mouvement vers le haut (Z/W)
                VirtualKeyCode::Up => self.player_y -= 1,
                // Mouvement vers le bas (S)
                VirtualKeyCode::Down => self.player_y += 1,
                // Ignorer le rest
                _ => {}
            }
        }
        
        //Nettoyer écran à chaque frame
        ctx.cls();
        
        ctx.print(1,1,"Roguelike Test");
        
        //Player
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
         
    //Initialiser (Joueur au millieu)
    let gs = State{
        player_x: 40,
        player_y: 25,
    };
    
    main_loop(context,gs)
}
