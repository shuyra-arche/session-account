#![no_std]
#![no_main]

#[mold::entry]
fn main() -> Result<(), env::Reason> {
    account_server::run().map_err(|why| {
        system_support::debug::put(why);
        25
    })
}
