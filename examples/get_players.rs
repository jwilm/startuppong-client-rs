extern crate startuppong;

fn main() {
    let account = startuppong::account_from_env().unwrap();
    let player_res = startuppong::get_players(&account).unwrap();
    let players = player_res.players();
    println!("{:?}", players);
}
