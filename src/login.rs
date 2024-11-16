use sf_api::{command::Command, SimpleSession};

pub async fn login_with_env() -> SimpleSession {
    let username = std::env::var("SF_USERNAME").unwrap();
    let password = std::env::var("SF_PASSWORD").unwrap();
    // let server: String: String = std::env::var("SF_SERVER").unwrap();

    let sessions = SimpleSession::login_sf_account(&username, &password)
        .await
        .unwrap();
    if sessions.len() != 1 {
        panic!(
            "Handling of multiple characters on one SSO account not \
             implemented yet ({} characters found).",
            sessions.len()
        );
    }
    let mut session = sessions.into_iter().next().unwrap();
    let _ = session.send_command(Command::Update).await; // probably dumb
    return session;
}
