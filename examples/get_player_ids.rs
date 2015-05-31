extern crate startuppong;

use startuppong::account_from_env;
use startuppong::get_players_ids;

fn main() {
    let account = account_from_env().unwrap();
    let ids = get_players_ids(&account, vec!["Collin G", "Joe W"]).unwrap();
    println!("{:?}", ids);
}
