//now store all the functions learned here
mod my_mods;
use my_mods::{hash_maps, excercise_hash,algo};


fn main() {
    //call imported function here
    hash_maps::hash_maps_creating_hashes();
    hash_maps::brut_hash_update();
    hash_maps::updating_hash_table();
    let xer = excercise_hash::mode_median(vec![1,1,2,3,4,5,6,7,8,9,10]);
    println!("{xer:?}");
    excercise_hash::company::create_employee();
    algo::fizzbuz(32);
}
