extern crate startuppong;

use startuppong::account_from_env;
use startuppong::add_match_with_names;

fn main() {
    let account = account_from_env().unwrap();
    add_match_with_names(&account, "oien", "arst").unwrap();
}
