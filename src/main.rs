use anyhow::Result;
use axum::Router;
use database::Database;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

mod components;
mod database;
mod endpoints;
mod page;

#[derive(Clone, Deserialize, Serialize)]
pub struct Employee {
    full_name: String,
    salary: u32,
    bonus: u32,
    rating: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::default();

    let router = Router::new()
        .nest("/", page::get_nest())
        .nest("/", endpoints::get_nest())
        .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:7880").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
