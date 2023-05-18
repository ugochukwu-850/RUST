/*
-- Given a list of integers, use a vector and return the median (when sorted, the value in the
middle position) and mode (the value that occurs most often; a hash map will be helpful
here) of the list.
*/

use crate::my_mods::algo::is_even;
use crate::my_mods::hash_maps::updating_hash_table;
use std::collections::HashMap;
use std::{collections::linked_list, fmt::DebugTuple, io};

pub fn mode_median(list: Vec<i64>) -> (f64, i64) {
    //fn to get the median
    let list2 = list.clone();

    fn median(mut list: Vec<i64>) -> i64 {
        list.sort();
        let len = list.len() as u64;

        //if the list length is odd then
        if !is_even(len) {
            //get the center number and return
            let center = len / 2 as u64;
            //cast to usize type since vectors only take that type
            list[center as usize] //and return
        } else {
            let center = {
                //get the two center numbers and return their half
                let center = len / 2 as u64;
                let center2 = { len / 2 } + 1 as u64;
                (center + center2) as f64 / 2.0 as f64
            };
            list[center as usize]
        }
    }

    fn mode(list: Vec<i64>) -> f64 {
        //first loop and create a map
        let mut tree = HashMap::new();
        let mut highest = (0, 0);
        for x in list {
            let count = tree.entry(x).and_modify(|e| *e += 1).or_insert(0);
            if count >= &mut highest.0 {
                highest = (*count, x);
            }
        }
        highest.1 as f64
    }
    (median(list) as f64, mode(list2) as i64) //default if nothing is created
}

/*Using a hash map and vectors, create a text interface to allow a user to add employee
names to a department in a company. For example, “Add Sally to Engineering” or “Add
Amir to Sales.” Then let the user retrieve a list of all people in a department or all people
in the company by department, sorted alphabetically.
*/

pub mod company {
    use std::{fmt::format, io};

    struct Admin {
        name: String,
        age: i32,
        salary: i128,
        isAdmin: bool,
    }

    struct Engineering {
        name: String,
        age: i32,
        salary: i128,
    }

    enum types {
        admin(Admin),
        engine(Engineering),
    }

    impl Engineering {
        fn create(name: String, age: i32, salary: i128) -> Engineering {
            Self { name, age, salary }
        }

        fn display(&self) -> String {
            let dp = format!(
                "Sector: Engineering \n Name: {} \n Age: {} \n Salary: {} \n",
                self.name, self.age, self.salary
            );
            println!("{dp}");
            dp
        }
    }

    impl Admin {
        fn create(name: String, age: i32, salary: i128) -> Admin {
            Self {
                name,
                age,
                salary,
                isAdmin: true,
            }
        }
        fn display(&self) -> String {
            let name = &self.name;
            let age = self.age;
            let salary = self.salary;

            let dp = format!("Sector: Admin \n Name: {name} \n Age: {age} \n Salary: {salary} \n");
            println!("{dp}");
            dp
        }
    }

    pub fn create_employee() {
        //on function call init all possible variable to hold info at runtime
        let mut employees = Vec::new(); //vectore to hold both admin and emginner type

        //a loop to run until user is satisfied with his work
        'root: loop {
            let mut name = String::new();
            let mut age = String::new();
            let mut salary = String::new();
            let mut choice = String::new();

            //input the type you choose
            print!("Input 'admin' or enginnering to add each type \n >>> ");

            //check for os errors
            match io::stdin().read_line(&mut choice) {
                Ok(n) => {
                    //no os errors
                    //print the type the admin has choosen

                    print!("You choose to create an {choice} user");

                    //check what they wanted and create it
                    if (choice.trim() == "admin".to_string()) || (choice.trim() == "engine") {
                        //create an admin or engi...r instance and save
                        print!("Name : \n"); //collecting data
                        io::stdin().read_line(&mut name).expect("error");
                        print!("Age : \n");
                        io::stdin().read_line(&mut age).expect("error");
                        print!("Salary : \n");
                        io::stdin().read_line(&mut salary).expect("error");

                        //parse age and salary as they are numbers
                        let age: i32 = match age.trim().parse() {
                            Ok(num) => num,
                            Err(_) => {
                                //on err print a message
                                println!("Please input a valid age %%Fatal: all temporary potenal inpts would lost%%");
                                continue;
                            }
                        };

                        let salary: i128 = match salary.trim().parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Please input a valid salary :: %%Fatal: all temporary potenal inpts would lost%%");
                                continue;
                            }
                        };
                        //if create types based on the choice
                        if choice.trim() == "admin" {
                            employees.push(types::admin(Admin::create(name.clone(), age, salary)));

                            //success message and choice
                            println!("You have succesfully created an employee");

                            println!("Input '1' to continue creating ------------ '2' to view all employees");

                            let mut action = String::new();
                            match io::stdin().read_line(&mut action) {
                                Ok(n) => {
                                    if action.trim() == "1".to_string() {
                                        //the user wants to continue startst the loop
                                        continue 'root;
                                    } else if action.trim() == "2" {
                                        //now read all the data in the vector
                                        print!("These are your employees\n");

                                        for x in &employees {
                                            match x {
                                                types::admin(y) => y.display(),
                                                types::engine(y) => y.display(),
                                            };
                                        }
                                        break 'root;
                                    } else {
                                        println!("Action {action}");
                                        break 'root;
                                    }
                                }
                                Err(err) => {
                                    print!("Error");
                                    break;
                                }
                            }
                        } else {
                            employees.push(types::engine(Engineering::create(
                                name.clone(),
                                age,
                                salary,
                            )));
                            println!("You have succesfully created an employee");

                            println!("Input '1' to continue creating ------------ '2' to view all employees");
                            let mut action = String::new();
                            match io::stdin().read_line(&mut action) {
                                Ok(n) => {
                                    if action.trim() == "1" {
                                        //the user wants to continue startst the loop
                                        continue;
                                    } else if action.trim() == "2" {
                                        //now read all the data in the vector
                                        print!("These are your employees\n");

                                        for x in &employees {
                                            match x {
                                                types::admin(y) => y.display(),
                                                types::engine(y) => y.display(),
                                            };
                                        }
                                        break 'root;
                                    } else {
                                        break 'root;
                                    }
                                }
                                Err(err) => {
                                    print!("Error");
                                    break;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
                Err(error) => println!("error: {error}"),
            }
        }
    }
}
