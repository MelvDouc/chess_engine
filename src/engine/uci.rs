use std::io::{stdin, stdout, Write};

use crate::game::{
    fen_string::{parse_fen, START_FEN},
    Position,
};

/// "The engine has stopped searching and found the move <move> best in this position.
///	The engine can send the move it likes to ponder on. The engine must not start pondering automatically.
///	This command must always be sent if the engine stops searching,
/// also in pondering mode if there is a 'stop' command, so for every 'go' command a 'bestmove' command is needed!
///	Directly before that the engine should send a final 'info' command with the final search information,
///	so the GUI has the complete statistics about the last search."
const COMMAND_BEST_MOVE: &str = "bestmove";

const COMMAND_FEN: &str = "fen";

/// Send after receiving the "uci" command to identify the engine.
const COMMAND_ID: &str = "id";

const COMMAND_INFO: &str = "info";

const COMMAND_GO: &str = "go";

/// Sent to ask the engine if it is done completing the previous command.
/// It should always be answered by a "readyok" command.
const COMMAND_IS_READY: &str = "isready";

const COMMAND_POSITION: &str = "position";

/// Sent to instruct the engine to shut down ASAP.
const COMMAND_QUIT: &str = "quit";

/// Sent after receiving an "isready" command to accept new commands.
const COMMAND_READY_OK: &str = "readyok";

const COMMAND_SET_OPTION: &str = "setoption";
const COMMAND_START_POSITION: &str = "startpos";

/// Sent to instruct the engine to stop the current search.
const COMMAND_STOP: &str = "stop";

/// Sent <u>once</u> on startup to instruct the engine to use the UCI protocol.
const COMMAND_UCI: &str = "uci";

/// Sent to inform the engine that the next search will be performed on a new game.
const COMMAND_UCI_NEW_GAME: &str = "ucinewgame";

/// Send after the "option" command to acknowledge UCI mode.
const COMMAND_UCI_OK: &str = "uciok";

pub(crate) fn communicate_with_gui() {
    let mut board_optional: OptionalBoard = None;

    loop {
        let mut gui_input = String::new();

        stdout().flush().expect("Couldn't flush STDOUT.");
        stdin()
            .read_line(&mut gui_input)
            .expect("Error reading input.");
        remove_ending_line_break(&mut gui_input);

        if gui_input.eq(COMMAND_UCI) {
            input_uci();
            continue;
        }

        if gui_input.starts_with(COMMAND_SET_OPTION) {
            input_set_option();
            continue;
        }

        if gui_input.starts_with(COMMAND_IS_READY) {
            input_ready_ok();
            continue;
        }

        if gui_input.starts_with(COMMAND_UCI_NEW_GAME) {
            input_uci_new_game();
            continue;
        }

        if gui_input.starts_with(COMMAND_POSITION) {
            board_optional = input_position(gui_input);
            continue;
        }

        if gui_input.starts_with(COMMAND_GO) {
            input_analysis(&board_optional);
            continue;
        }

        println!("Unrecognized command. Ignoring.");
    }
}

fn input_uci() {
    println!("{} name I_SUCK_AT_NAMING_THINGS", COMMAND_ID);
    println!("{} author Melvin Doucet", COMMAND_ID);
    println!("{}", COMMAND_UCI_OK);
}

fn input_set_option() {
    todo!()
}

fn input_ready_ok() {
    println!("{}", COMMAND_READY_OK);
}

fn input_uci_new_game() {
    todo!()
}

fn input_position(gui_input: String) -> OptionalBoard {
    // Syntax: position [fen <fen_string> | startpos ]  moves <move_1> .... <move_n>
    let parts = gui_input.split("\\s").collect::<Vec<&str>>();
    let subcommand = parts[1];

    if subcommand.eq_ignore_ascii_case(COMMAND_FEN) {
        // TODO: play moves
        let pos = parse_fen(parts[2]);
        return Some(pos);
    }

    if subcommand.eq_ignore_ascii_case(COMMAND_START_POSITION) {
        let pos = parse_fen(START_FEN);
        return Some(pos);
    }

    None
}

fn input_analysis(board_optional: &OptionalBoard) {
    match board_optional {
        Some(pos) => {
            crate::utils::debug::print_board(pos);
        }
        None => todo!(),
    };
}

fn remove_ending_line_break(str: &mut String) {
    if let Some('\n') = str.chars().next_back() {
        str.pop();
    }

    if let Some('\r') = str.chars().next_back() {
        str.pop();
    }
}

type OptionalBoard = Option<Position>;
