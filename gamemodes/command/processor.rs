// gamemodes/command/processor.rs
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use once_cell::sync::Lazy;

use omp::players::Player; // sesuaikan jika Player ada di crate lain
use omp::types::colour::Colour; // sesuaikan path Colour jika berbeda
use omp::core; // untuk Log

/// signature handler: (player, args, help) -> success(bool)
pub type CommandHandler = fn(Player, &[&str], bool) -> bool;

/// info per-command: handler + enabled flag + custom flags (u32)
pub struct CommandInfo {
    pub handler: CommandHandler,
    pub enabled: bool,
    pub flags: u32, // custom flags, default 0
}

// Registry global: semua nama (utama + alias) -> CommandInfo
pub static COMMANDS: Lazy<RwLock<HashMap<String, CommandInfo>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// Set untuk melacak nama command utama (bukan alias)
pub static MAIN_COMMANDS: Lazy<RwLock<HashSet<String>>> =
    Lazy::new(|| RwLock::new(HashSet::new()));

pub static HELP_ALIASES: Lazy<RwLock<HashSet<String>>> =
    Lazy::new(|| RwLock::new(HashSet::new()));

pub fn register_command(names: &[&str], handler: CommandHandler) {
    if names.is_empty() {
        return;
    }
    
    let mut map = COMMANDS.write().unwrap();
    let mut main_commands = MAIN_COMMANDS.write().unwrap();
    let mut help_set = HELP_ALIASES.write().unwrap();
    
    // Nama pertama dianggap sebagai command utama
    let main_command = names[0];
    main_commands.insert(main_command.to_string());
    
    for &name in names {
        map.insert(
            name.to_string(),
            CommandInfo {
                handler,
                enabled: true,
                flags: 0,
            },
        );
        
        // Cek jika ini adalah command help
        if name == "help" {
            help_set.insert(name.to_string());
        }
    }
}

pub fn parse_command(input: &str) -> Option<(&str, Vec<&str>)> {
    if !input.starts_with('/') {
        return None;
    }
    let mut parts = input[1..].split_whitespace();
    let command = parts.next()?;
    let args = parts.collect::<Vec<_>>();
    Some((command, args))
}

#[allow(unused)]
pub fn process_command(player: Player, name: &str, args: &[&str]) -> bool {
    core::Log(&format!("process_command: looking for '{}', args={:?}", name, args));

    // 1) Jika ini adalah help alias dan ada argumen -> handle help-request di sini
    {
        let help_set = HELP_ALIASES.read().unwrap();
        if help_set.contains(name) && !args.is_empty() {
            let target = args[0];
            core::Log(&format!("Detected help request for '{}'", target));

            // cari handler target
            let map = COMMANDS.read().unwrap();
            if let Some(info) = map.get(target) {
                // panggil handler target dengan help = true
                let remaining: Vec<&str> = if args.len() > 1 {
                    args[1..].to_vec()
                } else {
                    Vec::new()
                };
                core::Log(&format!("Calling handler '{}' with help=true", target));
                let _ = (info.handler)(player, &remaining, true);
                // anggap help-request berhasil jika handler ada
                return true;
            } else {
                player.send_client_message(
                    Colour::from_rgba(0xFF0000FF),
                    &format!("Command `{}` tidak ditemukan.", target),
                );
                return false;
            }
        }
    }

    // 2) Normal flow: cek handler langsung dan panggil dengan help = false
    {
        let map = COMMANDS.read().unwrap();
        if let Some(info) = map.get(name) {
            core::Log(&format!("Found handler for '{}', calling (help=false)...", name));
            let res = (info.handler)(player, args, false);
            core::Log(&format!("Handler for '{}' returned {}", name, res));
            return res;
        }
    }

    // 3) Jika tidak ditemukan, kirim suggestion dan return false
    let map = COMMANDS.read().unwrap();
    let mut best: Option<(String, usize)> = None;
    for key in map.keys() {
        let dist = levenshtein_distance(key, name);
        match &best {
            None => best = Some((key.clone(), dist)),
            Some((_, best_dist)) if dist < *best_dist => best = Some((key.clone(), dist)),
            _ => {}
        }
    }

    if let Some((suggestion, dist)) = best {
        if dist <= 3 {
            player.send_client_message(
                Colour::from_rgba(0xFF0000FF),
                &format!("Command `{}` tidak dikenal. Mungkin maksudnya `{}`?", name, suggestion),
            );
        } else {
            player.send_client_message(
                Colour::from_rgba(0xFF0000FF),
                &format!("Command `{}` tidak dikenal. Ketik /help untuk daftar command.", name),
            );
        }
    } else {
        player.send_client_message(
            Colour::from_rgba(0xFF0000FF),
            &format!("Command `{}` tidak dikenal. Ketik /help untuk daftar command.", name),
        );
    }

    false
}

/// Levenshtein distance (char-based)

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let n = a_chars.len();
    let m = b_chars.len();

    if n == 0 { return m; }
    if m == 0 { return n; }

    let mut prev: Vec<usize> = (0..=m).collect();
    let mut cur: Vec<usize> = vec![0; m + 1];

    for (i, ac) in a_chars.into_iter().enumerate() {
        cur[0] = i + 1;
        for (j, bc) in b_chars.iter().enumerate() {
            let cost = if ac == *bc { 0 } else { 1 };
            cur[j + 1] = std::cmp::min(
                std::cmp::min(prev[j + 1] + 1, cur[j] + 1),
                prev[j] + cost,
            );
        }
        prev.copy_from_slice(&cur);
    }

    cur[m]
}

// === Command Management API ===

#[allow(unused)]
pub fn enable_command(cmd: &str) {
    if let Some(info) = COMMANDS.write().unwrap().get_mut(cmd) {
        info.enabled = true;
    }
}

#[allow(unused)]
pub fn disable_command(cmd: &str) {
    if let Some(info) = COMMANDS.write().unwrap().get_mut(cmd) {
        info.enabled = false;
    }
}

#[allow(unused)]
pub fn is_command_enabled(cmd: &str) -> bool {
    COMMANDS
        .read()
        .unwrap()
        .get(cmd)
        .map(|i| i.enabled)
        .unwrap_or(false)
}

#[allow(unused)]
pub fn get_total_command_count() -> usize {
    MAIN_COMMANDS.read().unwrap().len()
}

#[allow(unused)]
pub fn get_enabled_command_count() -> usize {
    let main_commands = MAIN_COMMANDS.read().unwrap();
    let commands = COMMANDS.read().unwrap();
    
    main_commands.iter()
        .filter(|main_cmd| {
            commands.get(*main_cmd)
                .map(|info| info.enabled)
                .unwrap_or(false)
        })
        .count()
}

#[allow(unused)]
pub fn get_disabled_command_count() -> usize {
    let main_commands = MAIN_COMMANDS.read().unwrap();
    let commands = COMMANDS.read().unwrap();
    
    main_commands.iter()
        .filter(|main_cmd| {
            commands.get(*main_cmd)
                .map(|info| !info.enabled)
                .unwrap_or(false)
        })
        .count()
}

// Fungsi baru untuk mendapatkan total command termasuk alias
#[allow(unused)]
pub fn get_total_command_full() -> usize {
    COMMANDS.read().unwrap().len()
}

// Fungsi untuk mendapatkan semua alias dari sebuah command utama
#[allow(unused)]
pub fn get_command_aliases(main_command: &str) -> Vec<String> {
    let commands = COMMANDS.read().unwrap();
    let main_commands = MAIN_COMMANDS.read().unwrap();
    
    // Jika yang diminta adalah command utama, cari semua aliasnya
    if main_commands.contains(main_command) {
        commands.keys()
            .filter(|cmd_name| {
                // Ambil semua command yang memiliki handler sama dengan command utama
                // tetapi bukan command utama itu sendiri
                *cmd_name != main_command && 
                commands.get(*cmd_name).map(|info| 
                    std::ptr::eq(
                        info.handler as *const (), 
                        commands.get(main_command).unwrap().handler as *const ()
                    )
                ).unwrap_or(false)
            })
            .cloned()
            .collect()
    } else {
        Vec::new()
    }
}

// Fungsi untuk mengecek apakah sebuah nama adalah command utama
#[allow(unused)]
pub fn is_main_command(cmd: &str) -> bool {
    MAIN_COMMANDS.read().unwrap().contains(cmd)
}

// Fungsi untuk mengecek apakah sebuah nama adalah alias
#[allow(unused)]
pub fn is_command_alias(cmd: &str) -> bool {
    let commands = COMMANDS.read().unwrap();
    let main_commands = MAIN_COMMANDS.read().unwrap();
    
    commands.contains_key(cmd) && !main_commands.contains(cmd)
}

// Fungsi untuk mendapatkan command utama dari sebuah alias
#[allow(unused)]
pub fn get_main_command_from_alias(alias: &str) -> Option<String> {
    let commands = COMMANDS.read().unwrap();
    let main_commands = MAIN_COMMANDS.read().unwrap();
    
    if !commands.contains_key(alias) || main_commands.contains(alias) {
        return None;
    }
    
    // Cari command utama yang memiliki handler sama
    let alias_handler = commands.get(alias).map(|info| info.handler as *const ());
    
    main_commands.iter()
        .find(|main_cmd| {
            commands.get(*main_cmd).map(|info| 
                info.handler as *const () == alias_handler.unwrap()
            ).unwrap_or(false)
        })
        .cloned()
}

#[allow(unused)]
pub fn get_command_flags(cmd: &str) -> Option<u32> {
    COMMANDS.read().unwrap().get(cmd).map(|i| i.flags)
}

#[allow(unused)]
pub fn set_command_flags(cmd: &str, flags: u32) {
    if let Some(info) = COMMANDS.write().unwrap().get_mut(cmd) {
        info.flags = flags;
    }
}
