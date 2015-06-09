extern crate startuppong;

use startuppong::Account;
use startuppong::get_players_ids;

fn main() {
    let account = Account::from_env().unwrap();
    let ids = get_players_ids(&account, vec!["Collin G", "Joe W"]).unwrap();
    println!("{:?}", ids);
}
