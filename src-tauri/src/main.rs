// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use redis;
use redis::{Client, Commands, Connection};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct RedisClient {
    connection: Connection,
    client: Client,
}

impl RedisClient {
    pub fn init() -> RedisClient {
        let client = Client::open("redis://127.0.0.1").unwrap();
        let connection = client.get_connection().unwrap();
        return RedisClient { client ,connection };
    }
}

// fn main() {
//     get_keys();
// }
//
// fn get_keys() {
//     let mut redis_client: RedisClient = RedisClient::init();
//
//     //const REDIS_CLIENT: RedisClient = RedisClient::init();
//
//     let keys: Vec<String> = redis_client.connection.keys("*").unwrap();
//     for key in keys {
//         println!("{key}");
//     }
// }