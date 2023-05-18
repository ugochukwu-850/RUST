pub mod closure;
pub mod iterators;

use mail::*;

fn main() {
    run_inventory();
    closure::sub_main();
    iterators::sub_main();
}

fn run_inventory() {
    //init a var to hold the iventory
    let store = Inventory {
        //init the shirts field to a vec holding the possible shirts colors
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    //init an eexample user pref
    let user_pref1 = Some(ShirtColor::Red);
    //init an example giveaway
    /*
    user_pref above is a some enum holding the shirtcholor enum
    and below we have a giveaway set to the return of thestrcuct impl giveaway on the uer pref1
     */
    let giveaway1 = store.giveaway(user_pref1);

    //print the giveaway and the users pref
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    //do that again but with a user_pref null
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
