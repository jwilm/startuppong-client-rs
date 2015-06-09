extern crate startuppong;

use startuppong::Account;
use startuppong::add_match_with_names;

fn main() {
    let account = Account::from_env().unwrap();
    add_match_with_names(&account, "oien", "arst").unwrap();
}
