//**
//
//  Archiva Server
//  Griffin Dalby
//
//  Controls the backend of the archiva system, handling user input &
//  serving data back.
//
// */

// Extern
extern crate chrono;

// Modules
mod archiva_fs;

use std::{fs::{self}, net::SocketAddr, path::Path};
use axum::{
    routing::{get},
    Router,
};
use colored::{Colorize, ColoredString};
use crossterm::event::{self, Event};
use ratatui::{prelude::CrosstermBackend, text::Text, widgets::*, Frame, Terminal};

use archiva_fs::{validate_environment, Config};
use tokio::task;


// Settings


// Constants
// Variables
// Functions
// Archiva

async fn render_loop(mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) {
    loop {
        terminal
            .draw(|f: &mut Frame<'_>| {
                let f_area = f.area();
                let block = Block::default().title("Archiva").borders(Borders::ALL);
                f.render_widget(block, f_area);
            })
            .unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[tokio::main]
async fn main() {
    // Setup Logging
    let tag_archiva: ColoredString = "Archiva".purple();
    let tag_https: ColoredString = "HTTPS".green();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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
            .expect(&format!("Failed to create storage direcory at {}", config.storage.path));
        println!("Created storage directory at {}", config.storage.path)
    }

    // Setup Server
    let addr = SocketAddr::from(([127,0,0,1], config.server.port));
    let app = Router::new().route("/", get(move || async move { 
        let _=format!("[{}] Server established! (@ localhost:{})", tag_https, addr.port()); }));

    let app = app.into_make_service();

    let net_listener = tokio::net::TcpListener::bind(addr)
        .await.expect("Failed to bind net_listener!");

    // TUI Setup
    let mut terminal = ratatui::init();

    // Spawn tasks
    let ui_task = task::spawn(render_loop(terminal));
    let server_task = task::spawn(async move {
        axum::serve(net_listener, app)
            .await
            .expect("The server has crashed!")
    });

    tokio::select! {
        _ = ui_task => println!("UI Render Task has stopped!"),
        _ = server_task => println!("Server Task has stopped!"),
    }
}
