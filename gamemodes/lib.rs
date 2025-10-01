mod connection;
use connection::Database;

#[macro_use]
mod helper;

mod command;
use command::processor::{parse_command, process_command};
use omprs_command::command;

pub mod define;

mod server;
use server::{ServerRule};

use omp::{
    core::MaxPlayers,
    events::Events,
    main,
    players::{
        Player, PlayerCameraCutType, PlayerKeys, PlayerState, PlayerWeapon, WeaponSlotData,
    },
    register,
    textdraws::{TextDraw, TextDrawStyle},
    types::{
        colour::Colour,
        vector::{Vector2, Vector3},
    },
    vehicles,
};

struct PlayerData;

struct Harwana;

#[command(name = "clear", alias = ["clearchat", "cc"])]
fn clearchat(player: Player, _args: &[&str], help: bool) -> bool {
    if help {
        player.send_client_message(Colour::from_rgba(0xFFFFFFFF), "/clearchat is for clear chat log");
        true
    }
    else
    {
        for _ in 0..50 {
            player.send_client_message(Colour::from_rgba(0xFFFFFFFF), " ");
        }
        true
    }
}

#[command(name = "attack", alias = ["hit", "atk"])]
fn attack(player: Player, args: &[&str], help: bool) -> bool {
    if help {
        player.send_client_message(
            Colour::from_rgba(0xFFFFFFFF),
            "/attack digunakan untuk menyerang target",
        );
        true
    }
    else
    {
        if let Some(target) = args.get(0) {
            player.send_client_message(
                Colour::from_rgba(0xFFFFFFFF),
                &format!("Attacking {target}!"),
            );
            true
        } else {
            player.send_client_message(
                Colour::from_rgba(0xFF0000FF),
                "Usage: /attack <target>",
            );
            false
        }   
    }
}

#[command(name = "move")]
fn r#move(player: Player, args: &[&str], help: bool) -> bool {
    if help {
        player.send_client_message(
            Colour::from_rgba(0xFFFFFFFF),
            "/move digunakan untuk bergerak ke arah tertentu",
        );
        true
    }
    else
    {
        if let Some(dir) = args.get(0) {
            player.send_client_message(
                Colour::from_rgba(0xFFFFFFFF),
                &format!("Moving {dir}!"),
            );
            true
        } else {
            player.send_client_message(
                Colour::from_rgba(0xFF0000FF),
                "Usage: /move <direction>",
            );
            false
        }
    }
}

#[command(name = "help", alias = ["?", "h"])]
fn help(player: Player, _args: &[&str], _help: bool) -> bool {
    ClientMessage!(
        player,
        0xFFFFFFFF,
        "Available commands: /help, /attack, /move, /clear, /test",
    );
    true
}

impl Events for Harwana {
    fn on_player_connect(&mut self, player: Player) {
        ClientMessage!(
            player,
            0xFFFFFFFF,
            "{FFFFFF}Welcome to {88AA88}G{FFFFFF}rand {88AA88}L{FFFFFF}arceny",
        );

        ServerMessage!(player, "Memek");
        InfoMessage!(player, "Ygy");
        ErrorMessage!(player, "Test Error");
        WarningMessage!(player, "Mampus");
        SyntaxMessage!(player, "WARNING");
    }

    fn on_player_text(&mut self, player: Player, message: String) -> bool {
        log!("Player {} bilang {message}", player.get_id());
        false
    }


    fn on_player_command_text(&mut self, player: Player, message: String) -> bool {
        omp::core::Log(&format!("on_player_command_text: Player {} message='{}'", player.get_id(), message));
    
        if let Some((cmd, args_vec)) = parse_command(&message) {
            omp::core::Log(&format!("Parsed command '{}' args={:?}", cmd, args_vec));
    
            // args_vec is Vec<&str>, pass slice
            let args_ref: Vec<&str> = args_vec.iter().map(|s| *s).collect();
            process_command(player, cmd, &args_ref);
            return true;
        }
        false
    }

    fn on_player_spawn(&mut self, player: Player) {
        if player.is_npc() {
            return;
        }
    }

    fn on_player_death(&mut self, player: Player, killer: Option<Player>, _reason: i32) {
    }

    fn on_player_request_class(&mut self, player: Player, _class_id: i32) -> bool {
        if player.is_npc() {
            return true;
        }
        false
    }

    fn on_player_update(&mut self, player: Player) -> bool {
        if player.is_npc() {
            return true;
        }
        true
    }
}

#[main]
pub fn game_entry() -> Result<(), Box<dyn std::error::Error>> {
    // Buat runtime manual biar mirip Pawn lifecycle
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        // OnGameModeInit
        let db = Database::new("mysql://root:password@localhost:3306/test_db");

        match db.init().await {
            Ok(_) => {
                println!("Server ready! (set GameModeText dll.)");
            }
            Err(e) => {
                println!("-----------------------------------------------");
                println!("Gagal terhubung ke database: {}", e);
                println!("Server dalam mode maintenance...");
                println!("-----------------------------------------------");
                return; // stop kalau DB gagal
            }
        }

        ServerRule(); // Setup Server Rule
    
        let game = Harwana;
    
        register!(game);
    
        println!("Harwana Loaded");
    
        //register!(game);
        //register!(MyAuth); // Biadap Jembot
    
        log!("Max Player: {}", MaxPlayers());
        // OnGameModeExit
        db.shutdown().await.unwrap();
        println!("Database disconnected. Server shutting down.");
    });

    Ok(())
}
