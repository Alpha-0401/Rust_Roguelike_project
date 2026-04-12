use bracket_lib::prelude::*; // Outils pour le développement de roguelike

mod map; //Recherche de map.rs
use map::*; // Extraire tout ce que est publique dans map.rs
mod game; //Recherche de game.rs
use game::*; // Extraire tout ce que est publique dans game.rs

// Point d'entrée principal. Renvoie un BError en cas d'échec lors de l'ouverture de la fenêtre.
fn main() -> BError {

    // 1. Configuration de la fenêtre
    // Utilisation du "Builder" pour créer une fenêtre de terminal (80 colonnes par 50 lignes)
    let context = BTermBuilder::simple80x50()
        .with_title("Leave the dungeon")
        .build()?; // Construit la fenêtre. Le '?' permet de fermer le programme proprement s'il y a une erreur
        
    // 2. Génération initiale de la carte
    let depth = 1;
    // Appel de fonction depuis map.rs qui renvoie 4 éléments importants :
    // - map_gen : Le tableau contenant tous les murs, les sols et les escaliers.
    // - px, py : Les coordonnées X et Y du centre de la première salle.
    // - enemies_gen : Une liste (Vecteur) contenant les coordonnées des ennemis placés dans les autres salles.
    let (map_gen, px, py, enemies_gen) = new_map(depth);
         
    // 3. Initialisation de l'état du jeu (Game State)
    // Rassemble toutes les informations que le jeu doit mémoriser à chaque tour.
    let gs = State{
        player_x: px, //Coordonnées du joueur
        player_y: py,
        map: map_gen, // Design du niveau
        map_depth: depth, // numero du niveau
        game_won: false, // Pas gagné encore
        hp: 3, // Points de vie
        enemies: enemies_gen, //Liste des ennemis
    };
    
    // 4. Lancement du jeu
    main_loop(context,gs)
}


