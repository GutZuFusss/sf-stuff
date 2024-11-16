use sf_api::{
    gamestate::{
        fortress::{Fortress, FortressResourceType},
        GameState,
    },
    misc::EnumMapGet,
};

pub async fn run_fortress(gs: &GameState) {
    let fortress = gs.fortress.as_ref().unwrap();
    print_fortress_greeting(fortress);
}

pub fn print_fortress_greeting(fortress: &Fortress) {
    println!("\n#########################");
    println!("#  Fortress automation  #");
    println!("#########################\n");

    let wood = fortress.resources.get(FortressResourceType::Wood);
    let stone = fortress.resources.get(FortressResourceType::Stone);
    println!(
        "Wood:   {} / {}  --  Extractor: {} / {} (+{} / h)",
        wood.current,
        wood.limit,
        wood.production.last_collectable,
        wood.limit,
        wood.production.per_hour
    );
    println!(
        "Stone:  {} / {}  --  Extractor: {} / {} (+{} / h)",
        stone.current,
        stone.limit,
        stone.production.last_collectable,
        stone.limit,
        stone.production.per_hour
    );
}
