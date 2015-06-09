extern crate startuppong;

use startuppong::Account;

fn main() {
    let account = Account::from_env().unwrap();
    let player_res = startuppong::get_players(&account).unwrap();
    let players = player_res.players();
    println!("{:?}", players);
}
