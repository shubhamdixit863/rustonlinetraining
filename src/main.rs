use std::fmt::{Debug, Display};

fn print_me(data:i32){
    println!("The data is {}",data)

}

// Trait in rust is similar to interfaces in other languages
// the whole purpose of trait is to have the shared behaviour


// We implement something called as trait bounds
fn print_me_generic<T:Display+Debug+PartialEq+Copy>(data:T){
    println!("The data is {}",data)

}

fn print_me_generic2<T,U>(data:T,data2:U) where T:Display+Clone+Mover,U:Clone+Mover{
    println!("The data is {}",data)

}


// fn print_me_generic_2<T:Display,U:Display>(data:T,data2:U){
//     println!("The data is {}",data)
//
// }

/**

fn print_me_generic_i32(data:i32){
    println!("The data is {}",data)

}

fn print_me_generic_f32(data:f32){
    println!("The data is {}",data)

}

fn print_me_generic__&str(data:&str){
    println!("The data is {}",data)

}


**/

// struct Point{
//     x:i32,
//     y:f32
// }

trait Mover{
    fn walk(data: String) ->String;
}

#[derive(Debug)]
struct Animal {
    leg:i32,
    eyes:i32
}


impl Mover for Animal{
    fn walk(data: String) -> String {
        data

    }
}

fn only_mover<T:Mover>(data: T) ->T{
    data
}

#[derive(Debug)]
struct Point<T:Mover,U:Copy+Display>{
    x:T,
    y:U
}

impl <T,U> Point<T,U>{
    fn get_x(&self)->&T{
        return &self.x
    }
}

impl <T,U> Mover for Point<T,U>{
    fn walk(data: String) -> String {
        data
    }
}


// generics with enum


enum Data<T,U>{
    Good(T),
    Bad(U)
}


fn main() {

    // Generics  -- Basics
    // Generics -Advanced
    // Generics help us write reusable code
    // Functions,structs ,enums,traits ,implementations

    // print_me(9);
    //
    // print_me(9.0);
    // print_me("suuu");
    //
    // print_me_generic(9);
    //
    // print_me_generic(9.8);
    // print_me_generic("hello")

    // Monomorphisation
    //
    let point1=Point{
        x: 9,
        y: 9.0,
    };


    let point12=Point{
        x: "string",
        y: 9.0,
    };
    //
    //
    // let point12=Point{
    //     x: vec![1,2,3],
    //     y: 9.0,
    // };
    //
    // let ret=point12.get_x();
    // println!("{:?}",ret)
    //
    // let data:Data<String,i32>=Data::Good(String::from("hello there"));
    //
    // match data {
    //     Data::Good(y) => {
    //
    //
    //     }        Data::Bad(h) => {}
    // }



    //
    // let mv:Box<dyn Mover>=Box::new(Animal{
    //     leg: 2,
    //     eyes: 2,
    // // });
    // 
    // let an=Animal{
    //     leg: 0,
    //     eyes: 0,
    // };
  // let c=  only_mover(an);
  //  println!("{:?}",c); 

    let c=  only_mover(point1);
    println!("{:?}",c);
    
    // Smart Pointers part ,Box ,Rc

    
    // Advanced ,dynamic ,trait types 

}