
use omp::{
    events::Events,
    main,
    players::{
        Player,
    },
    register,
    types::{
        colour::Colour,
    },
};

struct Gamemodes;

impl Events for Gamemodes {
    fn on_player_connect(&mut self, player: Player) {
        player.send_client_message(Colour::from_rgba(0xFFFFFFFF), "Hello!");
    }

    fn on_player_text(&mut self, player: Player, message: String) -> bool {
        omp::core::Log(&format!("Player {} say {message}", player.get_id()));
        false
    }

    fn on_player_command_text(&mut self, player: Player, message: String) -> bool {
        omp::core::Log(&format!("Player {} use {}", player.get_id(), message));
    
        if message == "/test"
        {
            player.send_client_message(Colour::from_rgba(0xFFFFFFFF), "Test Good");
            omp::core::Log(&format!("Message send"));
            return true; 
        }
        false
    }

    fn on_player_spawn(&mut self, player: Player) {
        if player.is_npc() {
            return;
        }

        omp::core::Log(&format!("Player {} spawned", player.get_id()));
    }

    fn on_player_request_class(&mut self, player: Player, _class_id: i32) -> bool {
        if player.is_npc() {
            return true;
        }
        
        omp::core::Log(&format!("Player {} request: {}", player.get_id(), _class_id));
        false
    }
}

#[main]
pub fn game_entry() -> Result<(), Box<dyn std::error::Error>> {

    let game = Gamemodes;

    omp::core::Log(&format!("Gamemodes Loaded"));

    register!(game);

    Ok(())
}
