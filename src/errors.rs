#[derive(Debug, Clone)]
pub(crate) enum FENError {
    InvalidSquare(String),
    InvalidColor(String),
    InvalidBoard(String, char),
    InvalidCastlingRights(String),
    InvalidHalfMoveClock(String),
    InvalidFormat(String),
}
