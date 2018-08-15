extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

#[derive(Debug)]
enum Department {
    IT(String),
    CSE,
    ECE,
    EEE
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self) {
        self.width = 20;
    }
}

fn main() {

    let mut v = vec![1,2,3,4,5];
    println!("{:?}", v);

    let v1 = &mut v;
    println!("{:?}", v1);
    v1.push(6);

    let third: &u32 = &v1[2];


    let s1 = "Selva";
    println!("{}", &s1[0..5]);
    
    let num: u8 = 1;
    match num {
        1 => println!("1"),
        _ => ()
    }
    let my_department: Department = Department::IT(String::from("yayayayaa"));
    match my_department {
        Department::IT(message) => {
            println!("YAYA!!! {}", message);
        },
        Department::CSE => {
          println!("adad!!!");  
        },
        Department::ECE => {
          println!("adad1!!!");  
        },
        Department::EEE => {
          println!("adad2!!!");  
        }
    }
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Secret number is {}", secret_number);

    println!("Input the number");

    let tup: (i32, u32) = (-1, 2);
    let (x, y) = tup;
    println!("X is {}", x);

    println!("Y is {}", y);

    let mut rec1 = Rectangle {
        width: 100,
        height: 50
    };

    println!("{:?}", rec1);
    println!("{}", rec1.area());

    rec1.set_width();

    println!("{:?}", rec1);
    println!("{}", rec1.area());

   /* {
        let s1 = String::from("Selva");
        let s2 = &s1;
        println!("Line is {}", s1);
    }*/

    println!("get_number is {}", get_number());

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid input");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}

fn inc(rec2 : &mut Rectangle) {

}

fn get_number () -> u32 {
    1
}

fn updateVec (v3 : &mut Vec<u32>) {

}