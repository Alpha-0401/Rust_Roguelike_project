use bracket_lib::prelude::*;

//Définition de l'état du jeu
struct State {}

//Boucle principale
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        //Nettoyer écran à chaque frame
        ctx.cls();
        
        ctx.print(1,1,"Roguelike Test");
        
        //Player
        ctx.print_color(40,25, RGB::named(YELLOW), RGB::named(BLACK), "@");
    }
}

fn main() -> BError {

    //Configuration du contexte
    let context = BTermBuilder::simple80x50()
        .with_title("First Roguelike")
        .build()?;
    //Initialiser
    let gs = State{};
    main_loop(context,gs)
}
