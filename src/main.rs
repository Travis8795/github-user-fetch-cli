use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct GitUser {
    login: String,
    id: u32,
    avatar_url: String,
    following:  u32,
    followers: u32,
    // bio: Option<String>,
    repos_url : String,
    public_repos: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <github_username>", args[0]);
        std::process::exit(1);
    }
    
    let user_name = &args[1];
    let client: Client = reqwest::blocking::Client::new();

    let url: String = format!("https://api.github.com/users/{user_name}");
    
    let response = client
        .get(&url)
        .header("User-Agent", "rust-cli")
        .send()
        .expect("Failed to fetch user");
    
    let body_text = response.text().expect("Failed to read response");
    
    let user: GitUser = serde_json::from_str(&body_text).expect("Failed to parse user");

        println!("User: {}",user.login);
        println!("ID: {}",user.id);
        println!("avatar_url: {}",user.avatar_url);
        println!("The number of repositorys: {}",user.public_repos);
        println!("Repositorys url: {}",user.repos_url);
        println!("followers: {}",user.followers);
        println!("following: {}",user.following);
}
