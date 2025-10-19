//**
//
//  Archiva Server
//  Griffin Dalby
//
//  Controls the backend of the archiva system, handling user input &
//  serving data back.
//
// */

// Modules
mod archiva_fs;

use std::{fs::{self, File}, io::Write, net::SocketAddr, path::Path};
use axum::{
    routing::{get},
    Router,
};
use colored::{Colorize, ColoredString};

use archiva_fs::{validate_environment, Config};

// Settings


// Constants
// Variables
// Functions
// Archiva

#[tokio::main]
async fn main() {
    // Setup Logging
    let tag_archiva: ColoredString = "Archiva".purple();

    println!("\n{}", "      _                      __        _                 ".bright_purple());
    println!("{}", "     / \\                    [  |      (_)                ".bright_purple());
    println!("{}", "    / _ \\     _ .--.  .---.  | |--.   __  _   __  ,--.   ".bright_purple());
    println!("{}", "   / ___ \\   [ `/'`\\]/ /'`\\] | .-. | [  |[ \\ [  ]`'_\\ :  ".bright_purple());
    println!("{}", " _/ /   \\ \\_  | |    | \\__.  | | | |  | | \\ \\/ / // | |, ".bright_purple());
    println!("{}\n", "|____| |____|[___]   '.___.'[___]|__][___] \\__/  \\'-;__/ ".bright_purple());    
    println!("{} {} {}", "Made with".purple(), "♥".red(), "by Griffin Dalby".purple());
    
    // Setup Environment
    
    println!("[{}] Setting up environment.", tag_archiva);
    validate_environment();

    let config_path = Path::new("config.yml");
    let config_data: String = fs::read_to_string(config_path)
        .expect("Failed to read config.yml!");
    let config: Config = serde_yaml::from_str(&config_data)
        .expect("Invalid config.yml format!");

    if !config.eula_acceptance {
        println!("{}", "You must accept the Archiva EULA before running this program!".red().bold());
        std::process::exit(1);
    }

    if !Path::new(&config.storage.path).exists() {
        fs::create_dir_all(&config.storage.path)
            .expect("Failed to create storage direcory");
        println!("Created storage directory at {}", config.storage.path)
    }

    let app = Router::new().route("/", get(|| async { "Archiva server running!" }));
    let addr = SocketAddr::from(([127,0,0,1], config.server.port));

    println!("Listening on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
