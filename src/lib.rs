//! An API wrapper for startuppong.com
//!
//! The wrapper is implemented as a few module level functions for accessing the endpoints. All of
//! the JSON responses are represented with structs. The rustc_serialize crate is heavily relied on
//! to handle decoding of JSON responses.
//!
//! Sign up for an account at [startuppong.com](http://www.startuppong.com).
//!
//! # Examples
//! ```no_run
//! use startuppong::Account;
//! use startuppong::get_players;
//!
//! // Get the current leaderboard
//! let account = Account::new("account_id".to_owned(), "account_key".to_owned());
//!
//! // Make request to startuppong.com
//! let players_response = get_players(&account).unwrap();
//!
//! // Consume the response and get the list of players
//! let players = players_response.players();
//!
//! // Print out the leaderboard
//! for player in &players {
//!     println!("{} ({}) - {}", player.rank, player.rating, player.name);
//! }
//! ```
extern crate rustc_serialize;
extern crate hyper;
extern crate mime;

use std::env;
use std::io::Read;

use hyper::header::ContentType;
use rustc_serialize::json;

/// Error types, From impls, etc
pub mod error;
use error::ApiError;

/// An account is necessary to make requests against the API.
///
/// This struct holds your account ID and access key. It is a required argument to all of the API
/// methods.
#[derive(Debug, RustcEncodable, Clone)]
pub struct Account {
    api_account_id: String,
    api_access_key: String
}

impl Account {
    /// Create a new Account
    pub fn new(id: String, key: String) -> Account {
        Account {
            api_account_id: id,
            api_access_key: key
        }
    }

    /// Try and create an account from environment variables
    ///
    /// The environment variables needed are **STARTUPPONG_ACCOUNT_ID** and
    /// **STARTUPPONG_ACCESS_KEY**.
    pub fn from_env() -> Result<Account, std::env::VarError> {
        Ok(Account::new(try!(env::var("STARTUPPONG_ACCOUNT_ID")),
                        try!(env::var("STARTUPPONG_ACCESS_KEY"))))
    }

    /// Return a ref to the api_account_id
    pub fn id(&self) -> &str {
        &self.api_account_id[..]
    }

    /// Return a ref to the api_access_key
    pub fn key(&self) -> &str {
        &self.api_access_key[..]
    }
}

/// A person on the ladder
#[derive(Debug, RustcDecodable, Clone)]
pub struct Player {
    pub id: u32,
    pub rating: f32,
    pub rank: u32,
    pub name: String
}

/// The stats before and after a set
#[derive(Debug, RustcDecodable, Clone)]
pub struct Match {
    pub loser_rating_after: f32,
    pub winner_rating_after: f32,
    pub played_time: u64,
    pub loser_rank_after: u32,
    pub winner_name: String,
    pub winner_rank_before: u32,
    pub winner_rating_before: f32,
    pub loser_name: String,
    pub winner_id: u32,
    pub loser_rank_before: u32,
    pub loser_rating_before: f32,
    pub id: u64,
    pub winner_rank_after: u32,
    pub loser_id: u32
}

/// Wrapper for APIs returning a player list.
///
/// Use [`players()`](struct.GetPlayersResponse.html#method.players) to consume the response and
/// get the underlying `Vec<Player>`
#[derive(Debug, RustcDecodable)]
pub struct GetPlayersResponse {
    players: Vec<Player>
}

impl GetPlayersResponse {
    /// Consume the response and get underlying Vec
    pub fn players(self) -> Vec<Player> {
        self.players
    }
}

/// Wrapper for APIs returning a match list.
///
/// Use [`matches()`](struct.GetMatchesResponse.html#method.matches) to consume the response and
/// get the underlying `Vec<Match>`.
#[derive(Debug, RustcDecodable)]
pub struct GetMatchesResponse {
    matches: Vec<Match>
}

impl GetMatchesResponse {
    /// Consume the response and get underlying Vec
    pub fn matches(self) -> Vec<Match> {
        self.matches
    }
}


/// Get ids for players
///
/// Since the API doesn't provide any way to do the matching for us, we query the get_players
/// endpoint and do a linear search for each name in names. The returned ids can be used as
/// arguments to the add_match and other APIs requiring player IDs
pub fn get_players_ids(account: &Account, names: Vec<&str>) -> Result<Vec<u32>, ApiError> {
    let players = try!(get_players(account)).players();
    let mut ids = Vec::with_capacity(names.len());

    for name in &names {
        let len_before = ids.len();
        for player in &players {
            if player.name.contains(name) {
                ids.push(player.id);
                break;
            }
        }

        // If the length hasn't changed, no player names matched
        if ids.len() == len_before {
            return Err(ApiError::PlayerNotFound(name.to_string()));
        }
    }

    Ok(ids)
}

/// Return all players associated with the given account
///
/// Wraps `/api/v1/get_players`
pub fn get_players(account: &Account) -> Result<GetPlayersResponse, ApiError> {
    let url = format!("http://www.startuppong.com/api/v1/get_players\
                      ?api_account_id={}&api_access_key={}", account.id(), account.key());
    get::<GetPlayersResponse>(&url)
}

/// Return most recent matches on the given account
///
/// Wraps `/api/v1/get_recent_matches_for_company`
pub fn get_recent_matches_for_company(account: &Account) -> Result<GetMatchesResponse, ApiError> {
    let url = format!("http://www.startuppong.com/api/v1/get_recent_matches_for_company\
                      ?api_account_id={}&api_access_key={}", account.id(), account.key());
    get::<GetMatchesResponse>(&url)
}

/// Helper for retrieving a resource
///
/// `get` assumes that the http response is JSON formatted, and the parameterized type T
/// implements rustc_serialize::Decodable.
fn get<T>(url: &str) -> Result<T, ApiError>
    where T: rustc_serialize::Decodable {
    let mut client = hyper::Client::new();
    let mut res = try!(client.get(&url[..]).send());
    let mut body = String::new();
    try!(res.read_to_string(&mut body));
    Ok(try!(json::decode::<T>(&body)))
}

/// Add a match
///
/// Using the given `winner_id` and `loser_id`, create a new match. See
/// [add_match_with_names](fn.add_match_with_names.html) for a potentially easier to consume API.
/// This method wraps the `/api/v1/add_match` endpoint.
pub fn add_match(account: &Account, winner_id: u32, loser_id: u32) -> Result<(), ApiError> {
    let mut client = hyper::Client::new();
    let data = format!("api_account_id={}&api_access_key={}&winner_id={}&loser_id={}",
                       account.id(), account.key(), winner_id, loser_id);
    let url = "http://www.startuppong.com/api/v1/add_match";
    try!(client.post(&url[..])
               .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
               .body(&data).send());

    Ok(())
}

/// Add a match using the given names.
///
/// It is possible for name lookup to fail. If that happens, the Result will unwrap to a
/// `startuppong::ApiError::PlayerNotFound(String)` where the String is the first name not
/// resolved. Names are matched in a case-sensitive fashion. The first result with a valid sub
/// string match is accepted.
pub fn add_match_with_names(account: &Account, winner: &str, loser: &str) -> Result<(), ApiError> {
    let names = vec![winner, loser];
    let ids = try!(get_players_ids(account, names));
    add_match(account, ids[0], ids[1])
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::Account;
    use super::get_players as get_players_;
    use super::get_players_ids as get_players_ids_;

    #[test]
    fn get_players() {
        let account = Account::from_env().unwrap();
        let player_res = get_players_(&account).unwrap();
        let players = player_res.players();
        // The account has been preconfigured with at least two players
        assert!(players.len() >= 2);
    }

    #[test]
    fn get_players_ids() {
        let account = Account::from_env().unwrap();
        let ids = get_players_ids_(&account, vec!["arst", "oien"]).unwrap();
        println!("{:?}", ids);
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json;
    use super::GetPlayersResponse;
    use super::GetMatchesResponse;

    #[test]
    fn parse_get_players_response() {
        let raw = r#"{
          "players": [
            {
              "name": "Eshaan Bhalla",
              "rank": 1,
              "rating": 561.844467876031,
              "id": 89
            },
            {
              "name": "Collin Green",
              "rank": 2,
              "rating": 635.422989640755,
              "id": 55
            },
            {
              "name": "Joe Wilm",
              "rank": 3,
              "rating": 484.820167747424,
              "id": 60
            }
          ]
        }"#;

        let res = json::decode::<GetPlayersResponse>(raw).unwrap();
        let players = res.players();

        assert_eq!(players[0].name, "Eshaan Bhalla");
        assert_eq!(players[1].name, "Collin Green");
        assert_eq!(players[2].name, "Joe Wilm");
        assert_eq!(players[0].rank, 1);
        assert_eq!(players[0].id, 89);
    }

    #[test]
    fn parse_get_matches_response() {
        let raw = r#"{
          "matches": [
            {
              "loser_rating_after": 513.938174130505,
              "winner_rating_after": 635.422989640755,
              "played_time": 1432949959,
              "loser_rank_after": 5,
              "winner_name": "Collin Green",
              "winner_rank_before": 2,
              "winner_rating_before": 632.015809629857,
              "loser_name": "Michael Carter",
              "winner_id": 55,
              "loser_rank_before": 5,
              "loser_rating_before": 517.345354141403,
              "id": 1093,
              "winner_rank_after": 2,
              "loser_id": 58
            },
            {
              "loser_rating_after": 484.820167747424,
              "winner_rating_after": 632.015809629857,
              "played_time": 1432945408,
              "loser_rank_after": 3,
              "winner_name": "Collin Green",
              "winner_rank_before": 3,
              "winner_rating_before": 628.94100790458,
              "loser_name": "Joe Wilm",
              "winner_id": 55,
              "loser_rank_before": 2,
              "loser_rating_before": 487.894969472701,
              "id": 1092,
              "winner_rank_after": 2,
              "loser_id": 60
            },
            {
              "loser_rating_after": 628.94100790458,
              "winner_rating_after": 487.894969472701,
              "played_time": 1432945400,
              "loser_rank_after": 3,
              "winner_name": "Joe Wilm",
              "winner_rank_before": 4,
              "winner_rating_before": 480.798589900423,
              "loser_name": "Collin Green",
              "winner_id": 60,
              "loser_rank_before": 2,
              "loser_rating_before": 636.037387476858,
              "id": 1091,
              "winner_rank_after": 2,
              "loser_id": 55
            }
          ]
        }"#;

        let res = json::decode::<GetMatchesResponse>(raw).unwrap();
        let matches = res.matches();

        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].winner_name, "Collin Green");
        assert_eq!(matches[0].played_time, 1432949959);
        assert_eq!(matches[0].loser_rank_after, 5);
        assert_eq!(matches[0].loser_rank_before, 5);
        assert_eq!(matches[0].winner_id, 55);
        assert_eq!(matches[0].loser_id, 58);
        assert_eq!(matches[0].id, 1093);
    }
}
