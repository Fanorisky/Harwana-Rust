#[macro_use]
mod helper;

pub mod define;

mod spawns;
use spawns::SpawnLocations;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    time::Instant,
};

mod server;
use server::{ServerRule, MyAuth};

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

enum Cities {
    LosSantos,
    SanFierro,
    LasVenturas,
}

struct PlayerData {
    pub selected_city: Option<Cities>,
    pub last_city_selection_tick: Instant,
    pub has_city_selected: bool,
}

struct Harwana {
    players_data: HashMap<i32, PlayerData>,
    class_selection_helper_td: TextDraw,
    los_santos_td: TextDraw,
    san_fierro_td: TextDraw,
    las_venturas_td: TextDraw,
    spawn_locations: SpawnLocations,
}

impl Harwana {
    pub fn setup_char_selection(&self, player: &Player) {
        match self.players_data[&player.get_id()].selected_city {
            Some(Cities::LosSantos) => {
                player.set_interior(11);
                player.set_pos(Vector3::new(508.7362, -87.4335, 998.9609));
                player.set_facing_angle(0.0);
                player.set_camera_pos(Vector3::new(508.7362, -83.4335, 998.9609));
                player.set_camera_look_at(
                    Vector3::new(508.7362, -87.4335, 998.9609),
                    PlayerCameraCutType::Move,
                );
            }
            Some(Cities::SanFierro) => {
                player.set_interior(3);
                player.set_pos(Vector3::new(-2673.8381, 1399.7424, 918.3516));
                player.set_facing_angle(181.0);
                player.set_camera_pos(Vector3::new(-2673.2776, 1394.3859, 918.3516));
                player.set_camera_look_at(
                    Vector3::new(-2673.8381, 1399.7424, 918.3516),
                    PlayerCameraCutType::Move,
                );
            }
            Some(Cities::LasVenturas) => {
                player.set_interior(3);
                player.set_pos(Vector3::new(349.0453, 193.2271, 1014.1797));
                player.set_facing_angle(286.25);
                player.set_camera_pos(Vector3::new(352.9164, 194.5702, 1014.1875));
                player.set_camera_look_at(
                    Vector3::new(349.0453, 193.2271, 1014.1797),
                    PlayerCameraCutType::Move,
                );
            }
            None => {}
        }
    }

    pub fn setup_selected_city(&mut self, player: &Player) {
        let playerid = player.get_id();
        if self.players_data[&playerid].selected_city.is_none() {
            self.players_data
                .get_mut(&player.get_id())
                .unwrap()
                .selected_city = Some(Cities::LosSantos);
        }

        match self.players_data[&playerid].selected_city {
            Some(Cities::LosSantos) => {
                player.set_interior(0);
                player.set_camera_pos(Vector3::new(1630.6136, -2286.0298, 110.0));
                player.set_camera_look_at(
                    Vector3::new(1887.6034, -1682.1442, 47.6167),
                    PlayerCameraCutType::Move,
                );
                self.los_santos_td.show_for_player(player);
                self.san_fierro_td.hide_for_player(player);
                self.las_venturas_td.hide_for_player(player);
            }

            Some(Cities::SanFierro) => {
                player.set_interior(0);
                player.set_camera_pos(Vector3::new(-1300.8754, 68.0546, 129.4823));
                player.set_camera_look_at(
                    Vector3::new(-1817.9412, 769.3878, 132.6589),
                    PlayerCameraCutType::Move,
                );
                self.los_santos_td.hide_for_player(player);
                self.san_fierro_td.show_for_player(player);
                self.las_venturas_td.hide_for_player(player);
            }
            Some(Cities::LasVenturas) => {
                player.set_interior(0);
                player.set_camera_pos(Vector3::new(1310.6155, 1675.9182, 110.739));
                player.set_camera_look_at(
                    Vector3::new(2285.2944, 1919.3756, 68.2275),
                    PlayerCameraCutType::Move,
                );
                self.los_santos_td.hide_for_player(player);
                self.san_fierro_td.hide_for_player(player);
                self.las_venturas_td.show_for_player(player);
            }
            None => {}
        }
    }

    pub fn switch_to_next_city(&mut self, player: &Player) {
        match self.players_data[&player.get_id()].selected_city {
            Some(Cities::LosSantos) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::SanFierro);
            }
            Some(Cities::SanFierro) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LasVenturas);
            }
            Some(Cities::LasVenturas) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LosSantos);
            }
            None => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LosSantos);
            }
        }
        player.play_sound(1052, Vector3::default());
        self.players_data
            .get_mut(&player.get_id())
            .unwrap()
            .last_city_selection_tick = Instant::now();
        self.setup_selected_city(player);
    }

    pub fn switch_to_previous_city(&mut self, player: &Player) {
        match self.players_data[&player.get_id()].selected_city {
            Some(Cities::LosSantos) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LasVenturas);
            }
            Some(Cities::SanFierro) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LosSantos);
            }
            Some(Cities::LasVenturas) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::SanFierro);
            }
            None => {}
        }
        player.play_sound(1053, Vector3::default());
        self.players_data
            .get_mut(&player.get_id())
            .unwrap()
            .last_city_selection_tick = Instant::now();
        self.setup_selected_city(player);
    }

    pub fn handle_city_selection(&mut self, player: &Player) {
        let keydata = player.get_keys();
        if self.players_data[&player.get_id()].selected_city.is_none() {
            self.switch_to_next_city(player);
            return;
        }

        if self.players_data[&player.get_id()]
            .last_city_selection_tick
            .elapsed()
            .as_millis()
            < 500
        {
            return;
        }

        if (keydata.keys & PlayerKeys::FIRE) != 0 {
            self.players_data
                .get_mut(&player.get_id())
                .unwrap()
                .has_city_selected = true;
            self.los_santos_td.hide_for_player(player);
            self.san_fierro_td.hide_for_player(player);
            self.las_venturas_td.hide_for_player(player);
            player.toggle_spectating(false);
            return;
        }

        match keydata.leftRight.cmp(&0) {
            Ordering::Greater => self.switch_to_next_city(player),
            Ordering::Less => self.switch_to_previous_city(player),
            _ => {}
        }
    }
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
        self.players_data.insert(
            player.get_id(),
            PlayerData {
                selected_city: None,
                last_city_selection_tick: Instant::now(),
                has_city_selected: false,
            },
        );
    }

    fn on_player_text(&mut self, player: Player, message: String) -> bool {
        log!("Player {} bilang {message}", player.get_id());
        false
    }

    fn on_player_command_text(&mut self, player: Player, message: String) -> bool {
        log!("Player {} menggunakan {message}", player.get_id());
        true
    }

    fn on_player_spawn(&mut self, player: Player) {
        if player.is_npc() {
            return;
        }

        player.set_interior(0);
        player.toggle_clock(false);
        player.reset_money();
        player.give_money(30000);

        match self.players_data[&player.get_id()].selected_city {
            Some(Cities::LosSantos) => {
                let coords = self.spawn_locations.get_random_ls();
                player.set_pos(coords.0);
                player.set_facing_angle(coords.1);
            }
            Some(Cities::SanFierro) => {
                let coords = self.spawn_locations.get_random_sf();
                player.set_pos(coords.0);
                player.set_facing_angle(coords.1);
            }
            Some(Cities::LasVenturas) => {
                let coords = self.spawn_locations.get_random_lv();
                player.set_pos(coords.0);
                player.set_facing_angle(coords.1);
            }
            None => {}
        }

        player.give_weapon(WeaponSlotData::new(PlayerWeapon::Colt45, 100));
        player.toggle_clock(false);
    }

    fn on_player_death(&mut self, player: Player, killer: Option<Player>, _reason: i32) {
        self.players_data
            .get_mut(&player.get_id())
            .unwrap()
            .has_city_selected = false;
        if let Some(killer) = killer {
            let playercash = player.get_money();
            if playercash > 0 {
                killer.give_money(playercash);
                player.reset_money();
            }
        } else {
            player.reset_money();
        }
    }

    fn on_player_request_class(&mut self, player: Player, _class_id: i32) -> bool {
        if player.is_npc() {
            return true;
        }
        if self.players_data[&player.get_id()].has_city_selected {
            self.setup_char_selection(&player);
            return true;
        } else if player.get_state() != PlayerState::Spectating {
            player.toggle_spectating(true);
            self.class_selection_helper_td.show_for_player(&player);
            self.players_data
                .get_mut(&player.get_id())
                .unwrap()
                .selected_city = None;
        }
        false
    }

    fn on_player_update(&mut self, player: Player) -> bool {
        if player.is_npc() {
            return true;
        }

        //log!("Update");

        if !self.players_data[&player.get_id()].has_city_selected
            && player.get_state() == PlayerState::Spectating
        {
            self.handle_city_selection(&player);
            return true;
        }

        if player.get_weapon() == PlayerWeapon::Minigun {
            player.kick();
            return false;
        }
        true
    }
}

fn load_static_vehicles_from_file(path: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    for line in lines.map_while(Result::ok) {
        let mut seperator = line.split(',');
        let modelid: i32 = seperator.next().unwrap().parse()?;
        let x: f32 = seperator.next().unwrap().parse()?;
        let y: f32 = seperator.next().unwrap().parse()?;
        let z: f32 = seperator.next().unwrap().parse()?;
        let rotation: f32 = seperator.next().unwrap().parse()?;
        let colour1: i32 = seperator.next().unwrap().parse()?;
        let colour2: i32 = seperator
            .next()
            .unwrap()
            .split(' ')
            .next()
            .unwrap()
            .parse()?;

        vehicles::Vehicle::create_static(
            modelid,
            Vector3::new(x, y, z),
            rotation,
            colour1,
            colour2,
            30 * 60,
            false,
        );

        count += 1;
    }

    Ok(count)
}

fn create_city_name_td(city_name: &str) -> TextDraw {
    let td = TextDraw::create(Vector2::new(10.0, 380.0), city_name).unwrap();
    td.use_box(false);
    td.set_letter_size(Vector2::new(1.25, 3.0));
    td.set_style(TextDrawStyle::FontBeckettRegular);
    td.set_shadow(0);
    td.set_outline(1);
    td.set_color(Colour::from_rgba(0xEEEEEEFF));
    td
}

fn create_helper_td() -> TextDraw {
    let td = TextDraw::create(Vector2::new(10.0, 415.0), " Press ~b~~k~~GO_LEFT~ ~w~or ~b~~k~~GO_RIGHT~ ~w~to switch cities.~n~ Press ~r~~k~~PED_FIREWEAPON~ ~w~to select.").unwrap();
    td.use_box(true);
    td.set_box_color(Colour::from_rgba(0x222222BB));
    td.set_letter_size(Vector2::new(0.3, 1.0));
    td.set_text_size(Vector2::new(400.0, 40.0));
    td.set_style(TextDrawStyle::FontBankGothic);
    td.set_shadow(0);
    td.set_outline(1);
    td.set_background_color(Colour::from_rgba(0x000000FF));
    td.set_color(Colour::from_rgba(0xFFFFFFFF));
    td
}

#[main]
pub fn game_entry() -> Result<(), Box<dyn std::error::Error>> {
    ServerRule(); // Setup Server Rule

    let game = Harwana {
        class_selection_helper_td: create_helper_td(),
        los_santos_td: create_city_name_td("Los Santos"),
        san_fierro_td: create_city_name_td("San Fierro"),
        las_venturas_td: create_city_name_td("Las Venturas"),
        spawn_locations: SpawnLocations::new(),
        players_data: HashMap::new(),
    };

    println!("Harwana Loaded");

    register!(game);
    register!(MyAuth);

    log!("Max Player: {}", MaxPlayers());

    let vehicle_file_list = [
        "trains",
        "pilots",
        "lv_law",
        "lv_airport",
        "lv_gen",
        "sf_law",
        "sf_airport",
        "sf_gen",
        "ls_law",
        "ls_airport",
        "ls_gen_inner",
        "ls_gen_outer",
        "whetstone",
        "bone",
        "flint",
        "tierra",
        "red_county",
    ];

    let mut total_vehicles = 0;
    for file in vehicle_file_list {
        total_vehicles +=
            load_static_vehicles_from_file(&format!("scriptfiles/vehicles/{file}.txt"))?;
    }

    omp::core::Log(&format!("Total vehicles from files: {total_vehicles}"));

    Ok(())
}
