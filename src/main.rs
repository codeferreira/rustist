use clap::{Parser, Subcommand};
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    id: u64,
    name: String,
    comment_count: u32,
    order: Option<u32>,
    color: u32,
    shared: bool,
    sync_id: u32,
    favorite: bool,
    inbox_project: Option<bool>,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    projects: Vec<Project>,
}

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
}

fn main() {
    let mut tasks = Vec::new();
    tasks.push("Buy milk".to_string());
    tasks.push("Buy eggs".to_string());
    tasks.push("Buy bread".to_string());

    let value = Value::parse();
    match value.command {
        Commands::List => list_tasks(),
    }
}

#[tokio::main]
async fn list_tasks() {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.todoist.com/rest/v1/projects")
        .header(AUTHORIZATION, "Bearer [API_TOKEN]")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // println!("Status: {:?}", response.text().await);

            match response.json::<Vec<Project>>().await {
                Ok(parsed) => {
                    for project in parsed {
                        println!("{}", project.name);
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
        _ => println!("Error"),
    }
}
