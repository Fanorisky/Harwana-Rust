use omp::{
	core::{
		AddRule,
        DisableInteriorEnterExits, EnableStuntBonusForAll, SetGameModeText,
        SetNameTagsDrawDistance, SetWeather, SetWorldTime, ShowNameTags, ShowPlayerMarkers, UsePedAnims,
    }
};

use omp::events::Events;
use omp::players::Player;
use omp::types::colour::Colour;

// bikin struct sendiri
pub struct MyAuth;

// implementasi Events untuk struct MyAuth
impl Events for MyAuth {
    fn on_player_connect(&mut self, player: Player) {
        // contoh minimal: kirim pesan ke player saat connect
        player.send_client_message(Colour::from_rgba(0x77ff0000), "Jembut!");
    }
}

#[allow(non_snake_case)]
pub fn ServerRule()
{
	SetGameModeText("Harwana Rust");
    ShowPlayerMarkers(0);
    ShowNameTags(true);
    SetNameTagsDrawDistance(40.0);
    EnableStuntBonusForAll(false);
    DisableInteriorEnterExits();
    UsePedAnims();
    SetWeather(2);
    SetWorldTime(11);
	AddRule("developer", "Fanorisky");
}