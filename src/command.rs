// src/command.rs
use std::collections::HashMap;
use omp::players::Player;
use omp::types::colour::Colour;

type CommandFn = fn(Player, Vec<String>, Colour);

pub struct CommandProcessor {
    commands: HashMap<String, CommandFn>,
}

impl CommandProcessor {
    pub fn new() -> Self {
        Self { commands: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, func: CommandFn) {
        self.commands.insert(name.to_string(), func);
    }

    pub fn process(&self, player: Player, message: &str, default_colour: Colour) -> bool {
        if !message.starts_with('/') {
            return false;
        }

        let parts: Vec<String> = message[1..]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if parts.is_empty() {
            return false;
        }

        let cmd = &parts[0];
        let args = parts[1..].to_vec();

        if let Some(handler) = self.commands.get(cmd) {
            handler(player, args, default_colour);
            true
        } else {
            false
        }
    }
}

// contoh command bawaan
pub fn default_commands(processor: &mut CommandProcessor) {
    processor.register("help", |player, _args, colour| {
        player.send_client_message(colour, "Available commands: /help, /spam");
    });

    processor.register("spam", |player, args, colour| {
        let times: usize = args.get(0).and_then(|x| x.parse().ok()).unwrap_or(5);
        for i in 0..times {
            player.send_client_message(colour, &format!("Spam #{i}"));
        }
    });
}
