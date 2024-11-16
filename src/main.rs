mod fortress;
mod login;

use sf_api::gamestate::character::Character;

#[tokio::main]
pub async fn main() {
    let session = login::login_with_env().await;

    let gs = session.game_state().unwrap();
    let char = gs.character.clone();
    greet_character(char);
}

fn greet_character(char: Character) {
    println!(
        "Logged in as {} (Level:{} -XP: {}/{})",
        char.name, char.level, char.experience, char.next_level_xp
    );
    println!("Shrooms: {}", char.mushrooms);
    println!("Silver: {}", char.silver);
}
