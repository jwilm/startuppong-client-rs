extern crate hyper;
extern crate rustc_serialize;

use std::io;
use std::fmt;
use std::convert::From;
use std::error::Error;

use rustc_serialize::json;

/// The error type returned in a startuppong `Result`.
#[derive(Debug)]
pub enum ApiError {
    /// An ID was not found for the given player name in a call to `get_player_ids`
    PlayerNotFound(String),
    /// Something went wrong during the request
    Http(hyper::error::Error),
    /// Error reading response
    Io(io::Error),
    /// Response JSON could not be decoded
    JsonDecoding(json::DecoderError)
}

impl Error for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::PlayerNotFound(_) => "Could not match player name to id",
            ApiError::Http(ref err) => err.description(),
            ApiError::Io(ref err) => err.description(),
            ApiError::JsonDecoding(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApiError::PlayerNotFound(_) => None,
            ApiError::Http(ref err) => Some(err),
            ApiError::Io(ref err) => Some(err),
            ApiError::JsonDecoding(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::PlayerNotFound(ref s) => write!(f, "PlayerNotFound: {}", s),
            ApiError::Http(ref err) => write!(f, "Http error: {}", err),
            ApiError::Io(ref err) => write!(f, "Io error: {}", err),
            ApiError::JsonDecoding(ref err) => write!(f, "JsonDecoding error: {}", err),
        }
    }
}

impl From<hyper::error::Error> for ApiError {
    fn from(err: hyper::error::Error) -> ApiError {
        ApiError::Http(err)
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> ApiError {
        ApiError::Io(err)
    }
}

impl From<json::DecoderError> for ApiError {
    fn from(err: json::DecoderError) -> ApiError {
        ApiError::JsonDecoding(err)
    }
}
