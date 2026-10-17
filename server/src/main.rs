//! RetroVerse server by Navine: a base project for a game server.
//!
//! This starts, opens a SQLite database and answers a health check. Accounts, profiles, chat,
//! moderation and the game-facing endpoints are stubs with the intended shape: building and
//! running this crate gives a server that does nothing useful beyond proving it is up.

mod api;
mod auth;
mod db;
mod mcp;
mod moderation;
mod og;
mod xmpp;

#[tokio::main]
async fn main() {
    let db_path = db::default_path();
    match db::open(&db_path) {
        Ok(_) => println!("database ready at {}", db_path.display()),
        Err(error) => {
            eprintln!("could not open database: {error}");
            std::process::exit(1);
        }
    }

    let app = api::router();
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|error| panic!("could not bind {addr}: {error}"));
    println!("retroverse-server listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
