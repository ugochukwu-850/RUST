//to have sub modules or crates 
//create a cache {name of present file } directory and store the file.rs in that dir

use std::vec;

mod cars;

use cars::car;

#[derive(Debug)] //deriving of the debug

//a rectangle data structure  know as class
struct Rectangle {
    width: u32,
    height: u32,
}

//implementing a class function for the reactangle
impl Rectangle {
    //class function area takes as reference the structs self var
    //and returns an unsigned interger of 32 bits
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method body called here
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Naira,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn coin_match(coin: Coin) -> i32 {
    match coin {
        Coin::Dime => {
            println!("You just got a Dime");
            1
        }
        Coin::Naira => {
            println!("You just got a Dime");
            12
        }
        Coin::Nickel => {
            println!("You just got a Dime");
            5
        }

        Coin::Quarter(state) => {
            println!("You got this coin from {:?}", state);
            34
        }
        _ => 2,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    /*init a new rectangle
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let new = Rectangle::square(3);
        println!(
            "The area of the rectangle is {} square pixels.  And one morenew rect {}",
            rect1.area(),
            new.area() //calling its area function
        );

        let m = Message::Write(String::from("Good try"));
        m.call();

        let x = Some(5);
        let y: Option<i32> = None;
        let y = 12;

        print!("{y}\n")

        println!("{}", coin_match(Coin::Penny));

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);


        let mut vec = Vec::new();
        vec.push(2);
        vec.push(3);
        vec.push(2);

        let xer = vec.contains(&2);
        print!("{xer:?}\n");


        let new =  vec.get(0);
        print!("{new:?}\n");
    */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = { format!("{s1} it would {s2} it should ").replace("world", "Chuks") }; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    let new_car = car::create("Benz".to_string());
    println!("{}", new_car.name);
    let x  = cars::company::subcompany::third_gen::third_generation();
    println!("{}", x);
    let resc = cars::company::company_test();
}
