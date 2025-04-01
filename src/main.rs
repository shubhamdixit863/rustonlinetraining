use std::env;
use std::fmt::{Display, Formatter};
//
//
// trait  Data{
//     fn data(&self);
// }
//
// struct FileData;
// impl Data for FileData{
//     fn data(&self) {
//        println!("{}","something data")
//     }
// }
//
// fn data<T:Data>(d:T){
//     d.data();  // static dispatch
//     println!("{}","hell data generic function")
//
// }
//
// trait Vehcile{
//     fn drive(&self);
// }
// struct Car;
//
// impl Vehcile for Car{
//     fn drive(&self) {
//         println!("Car method gets called")
//     }
// }
// fn foo(t:impl Vehcile){
//     t.drive()
// }
//
// fn main() {
//
//     // data(FileData);
//     //
//     // foo(Car);
//
//     let v:Vec<Box<dyn Vehcile>>;
//
// }

// Cache Eviction
// LRU ,MFU

trait EvictionPolicy{
    fn evict(&self);
}

struct  LFU<T>{
    data:T
}


impl <T> EvictionPolicy for LFU<T> {
    fn evict(&self) {
        println!("{}","LFU Evicted");
    }
}

struct MFU <T>{
    data:T
}
impl <T> EvictionPolicy for MFU<T> {
    fn evict(&self) {
        println!("{}","MFU Evicted");
    }
}


// super trait 

trait  Printable{
    fn print(&self);
}

trait  Shape:Printable+Display+Clone{
    fn area(&self);
    fn print(&self);
    
}


struct Circle;

impl Printable for Circle {
    fn print(&self) {
        todo!()
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Shape for Circle{
    fn area(&self) {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}


// marker traits
trait Marker{}

struct MyType;

impl Marker for MyType{}

fn check<T:Marker>(_:T){
    
}

// Associated types 
// Generic traits
// ----Box ,RC Pointer , --->Implemt custom Linked  List  ,Weak Refs


// Concurrency section

// dynamic dispatch
fn main() {
    // let lfu=Box::new(
    //     LFU{
    //         data: "I am the lfu data",
    //     }
    // );
    // 
    // let mfu=Box::new(
    //     MFU{
    //         data: 89
    //     }
    // );
    // let v:Vec<Box<dyn EvictionPolicy >>=vec![mfu,lfu];
    // 
    // for i in v{
    //     i.evict()
    // }
    let args: Vec<String> = env::args().collect();
    let c:i32=args[1].parse().unwrap();
    
    // let c: dyn EvictionPolicy =MFU{
    //     data: 9,
    // };
    // let c:Box<dyn EvictionPolicy>=Box::new(
    //     MFU{
    //         data: (),
    //     }
    // );

    
    // let mut s:&dyn EvictionPolicy;
    // 
    // // s is a fat pointer
    // 
    // if c>8{
    //     s=&MFU{
    //         data:9,
    //     }
    // }else{
    //     s=&LFU{
    //         data:9,
    //     }
    // }
    // 
    // s.evict();
    
    check(MyType);

}