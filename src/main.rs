#![allow(dead_code)]
#![allow(unused_variables)]
mod sh;
use std::mem;
const MY_NAME: &str = "UWENAYO Alain Pacifique";
static VERSION: &'static str ="1.0.3";
fn print_my_name(s: &str) {
    if s == MY_NAME {
    println!("My name is {}",MY_NAME);
    }else {
        println!("My name is not {} but {}",s,MY_NAME);
    }
}

fn while_loop(){
    // continue is used to skip to the second iteration
    //break is used to terminate the loop
    let mut x:u64 =1;
    while x <10 {
        x=x+1;
    println!("{}",x);
    }
    let mut y = 1;
    loop  {
        y*=2;
        println!("{}",y);
        if y == 1<<10 {break;}
    }
}

fn version_control(v: &str){
     let mut x:bool = false;
    return println!("{}" ,if v == VERSION {x=!x;x} else {x})
}

fn others(){
    let a = 127;
    println!("a = {}, size ={} bits",a,mem::size_of_val(&a)*8);
    let pi = std::f64::consts::PI;
    let cubed = i32::pow(a,3);
    println!("{} and {}",pi,cubed);
    let two_to_10 = 1 << 10;
    println!("2^5 and {}",two_to_10);
}
fn match_statements(){
    let country_code =100;
    let country = match country_code {
        44 => "United States Of America",
        100 =>"Rwanda",
        30 =>"Congo",
        1..=1000 =>"unknown",
        _ => "invalid",
    };
    println!("the country with {} country code is {}",country_code,country);
}
fn main() {
    // unsafe is used to update the static variables --it's not safe
    // unsafe {
    //     VERSION = "23.3.5";
    //     version_control(VERSION);
    // }
    // print_my_name("UWENAYO Alain Pacifique");
    // version_control(VERSION);
    // others()
    // sh::heap_and_stack_memory()
    // while_loop();
    match_statements();
}