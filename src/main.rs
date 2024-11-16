mod fortress;
mod login;

use sf_api::gamestate::character::Character;

#[tokio::main]
pub async fn main() {
    let session = login::login_with_env().await;

    let gs = session.game_state().unwrap();
    let char = gs.character.clone();
    greet_character(char);

    // for now, theres just fortress automation
    fortress::run_fortress(&gs).await;
}

fn greet_character(char: Character) {
    println!(
        "Logged in as {} (Level: {} - XP: {}/{})",
        char.name, char.level, char.experience, char.next_level_xp,
    );
    println!("Shrooms:     {}", char.mushrooms);
    println!("Gold:        {}", char.silver / 100);
    println!("Rank:        {}", char.rank);
    println!(
        "Scrapbook:   {}",
        char.scrapbok.as_ref().unwrap().items.len()
            + char.scrapbok.as_ref().unwrap().monster.len()
    );
}
