//bring std into scope
use minigrep::*;
use std::env;
use std::process;
//main function to run all
fn main() {
    /*let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("An error occured while parsnig file ==> {err}");
        process::exit(1);
    });
    updating with knowledge from iterators
    */
    let config = Config::build(
        env::args(), /*pass the build function the iter from enc  instead*/
    )
    .unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //print the file context
    if let Err(e) = run(config) {
        eprintln!("That is an error !!! {e} \n");
        process::exit(1)
    };
}
