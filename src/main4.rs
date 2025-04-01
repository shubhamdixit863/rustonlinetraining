
// Trait bounds ,static dispatch vs dynamic dispatch


trait Data<T>{
    fn read(d:T);
}


struct MyReader;

impl Data<String> for MyReader {
    fn read(d: String) {
        println!("Reading a String: {}", d);
    }
}

impl Data<i32> for MyReader {
    fn read(d: i32) {
        println!("Reading an i32: {}", d);
    }
}


// // static dispatch
// trait Data {
//     fn some_trait_fn(&self);
// }
//
// struct FileData{
//
// }
//
// impl Data for FileData {
//     fn some_trait_fn(&self) {
//         println!("{}","I got printed") ; // default implementation
//     }
// }
//
// fn read_it<T:Data+Discover+Move>(data:T) where T:Data{
//        data.some_trait_fn();
// }

// static dispatch using impl keyword

// use std::fmt::Debug;
//
// trait Shape{
//      fn area(&self);
// }
//
//
// struct Circle;
//
// impl Shape for Circle{
//     fn area(&self) {
//         println!("{}","Circle area")
//     }
// }
// fn print_shape(s:impl Shape){
//      s.area();
// }


// Dynamic dispatch --is more a runtime thing

use std::fmt::Debug;

use crate::cache::cache_impl::EvictionPolicy;

mod cache;

trait Draw{
    fn data(&self);
}


#[derive(Debug)]
struct Circle;

impl  Draw for Circle {
    fn data(&self) {
       println!("{}","for circle");
    }
}

struct Square;
impl Draw for Square {
    fn data(&self) {
        println!("{}","for square");
    }
}


// dyanamic dispatch
fn draw_all(shapes:Vec<Box<dyn Draw>>){
    for shape in shapes{
        shape.data();
    }

}

// Cache Eviction Strategy  // LRU // MRU  ,LFU ,FIFO

// strategy design pattern;
// fn cache_eviction(policies: Vec<Box<dyn EvictionPolicy>>){
//     for policy in policies{
//         policy.evict();
//     }
// 
// 
// }

fn cache_eviction(policy: Box<dyn EvictionPolicy>){
    policy.evict();

}

fn cache_eviction_2(policies: Vec<Box<dyn EvictionPolicy>>){
    for policy in policies{
        policy.evict()
    }

}

fn main() {

    // let fd=FileData{
    //
    // };
    //
    // read_it(fd)

    // let ci=Circle;
    // print_shape(ci);
    // let sq=Square;
    // let c=Circle;
    //
    // let vec:Vec<Box<dyn Draw>>=vec![Box::new(sq),Box::new(c)];
    // draw_all(vec);

    let lru1=cache::cache_impl::LRU::new(7);
    let lru2=cache::cache_impl::LRU::new(Circle);
    let lru3=cache::cache_impl::LRU::new(String::from("the string data"));
    let lfu=cache::cache_impl::LFU::new("string data");


    
    //
     let v:Vec<Box<dyn EvictionPolicy>>=vec![lru1,lru2,lru3,lfu];

    cache_eviction_2(v);
 
// trait types
    
}