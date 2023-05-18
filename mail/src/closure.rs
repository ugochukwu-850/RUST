use std::{thread, iter::Enumerate};

pub fn sub_main() {
    closure_mutability_laws();
    println!("move closure");
    making_closures_own_the_input();
    sort_rect();
    println!("Contradiction sort runninng ....");
    contradicting_sort_rect();
}

///potrays closure_mutability law

pub fn closure_mutability_laws() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    //println!("{:?}", list); //err reference rule no two mutable ref at once

    borrows_mutably();
    //print always can mutate your vars so its a mututive macro function
    println!("After calling closure: {:?} {:?}", list.push(1), list);
}

pub fn making_closures_own_the_input() {
    //init list or vec
    let list = vec![1, 2, 3];
    //mutaby write it out
    println!("Before defining closure: {:?}", list);
    //call a thread to spawn a forced move of to the closure ""ref(1)"" that prints from thred
    thread::spawn(move /*-move ==> err thread might outlive parent
         and depends on parent for a ref */ || /*the before is the closure */ println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//impl sorting a struct with a key
fn sort_rect() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    /*
            The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple
        times: once for each item in the slice. The closure |r| r.width doesn’t capture, mutate, or
        move out anything from its environment, so it meets the trait bound requirements.
            alloc::slice
    pub fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
        __function declaration shows it
        can only work on types which fufil the Fnmut trait since it has to call the type multiple times
        as it runs through the sorting algorithm
            */
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}


///This is a contrived, convoluted way (that doesn’t work) to try and count the number of times
///sort_by_key gets called when sorting list . This code attempts to do this counting by
///pushing value —a String from the closure’s environment—into the sort_operations vector.
///The closure captures value then moves value out of the closure by transferring ownership of
///value to the sort_operations vector. This closure can be called once; trying to call it a
///second time wouldn’t work because value would no longer be in the environment to be
///pushed into sort_operations again! Therefore, this closure only implements FnOnce . When
///we try to compile this code, we get this error that value can’t be moved out of the closure
///because the closure must implement FnMut :
fn contradicting_sort_rect() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");
    list.sort_by_key(|r| {
        sort_operations.push(value.clone()); //here value is prisoned andnever release || adding a clone would solve it
                                     //on sec iter ther is no a value to be imprisoned soan err
                                     //but the most reasonable approach is to use a counter
        r.width
    });
    println!("{:#?}", list);
    println!("{:#?}",sort_operations.len()) //prove that cloning would work
}

/*
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
    F: FnOnce() -> T
    {
    match self {
    Some(x) => x,
    None => f(),
    }
    }
    }

    __How the class function unwrap_or_else works
    1 - Takes in a genric type <any thing>
    2 - Returns a Generic tupe
    3 - signatures and type anotation on where the genric type implements the FnOnce trait native to closures
    that means the genric type must impl the trait of being able to be called once and returns type T
    OR more professionally
    "The trait bound specified on the generic type F is FnOnce() -> T , which means F must be
    able to be called once, take no arguments, and return a T ."
    4 - macth the self  ==> if self macthes Some valid input not None  => return the input
    5 - if the input is none return the f function or closure > this is possible cause of
    the returning of genric type
    */
