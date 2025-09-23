
macro_rules! log { // same as printf in pawn
    ($string:literal) => {
        omp::core::Log(&format!($string));
    };
    ($string:literal,$($args:expr),*) => {
        omp::core::Log(&format!($string,$($args),*));
    };
}

macro_rules! ClientMessage {
    // tanpa argumen format, trailing comma optional
    ($player:expr, $color:expr, $string:expr $(,)?) => {
        $player.send_client_message(
            Colour::from_rgba($color),
            &format!("{}", $string),
        );
    };

    // dengan argumen format
    ($player:expr, $color:expr, $string:expr, $($args:expr),+ $(,)?) => {
        $player.send_client_message(
            Colour::from_rgba($color),
            &format!($string, $($args),*),
        );
    };
}

macro_rules! ServerMessage {
    // tanpa argumen format, trailing comma opsional
    ($player:expr, $string:expr $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::SERVER_CM, $string)
        );
    };

    // dengan argumen format
    ($player:expr, $string:expr, $($args:expr),+ $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::SERVER_CM, format!($string, $($args),*))
        );
    };
}

macro_rules! InfoMessage {
    ($player:expr, $string:expr $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::INFO_CM, $string)
        );
    };

    ($player:expr, $string:expr, $($args:expr),+ $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::INFO_CM, format!($string, $($args),*))
        );
    };
}

macro_rules! ErrorMessage {
    ($player:expr, $string:expr $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::ERROR_CM, $string)
        );
    };

    ($player:expr, $string:expr, $($args:expr),+ $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::ERROR_CM, format!($string, $($args),*))
        );
    };
}

macro_rules! WarningMessage {
    ($player:expr, $string:expr $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::WARN_CM, $string)
        );
    };

    ($player:expr, $string:expr, $($args:expr),+ $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::WARN_CM, format!($string, $($args),*))
        );
    };
}

macro_rules! SyntaxMessage {
    ($player:expr, $string:expr $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::SYNTAX_CM, $string)
        );
    };

    ($player:expr, $string:expr, $($args:expr),+ $(,)?) => {
        ClientMessage!(
            $player,
            0xFFFFFFFF,
            format!("{}{}", define::SYNTAX_CM, format!($string, $($args),*))
        );
    };
}
