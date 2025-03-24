

// struct Book<'a>{
//     title:&'a str
// }

//Write a struct X with two fields: s (an Option<String> ) and i (an i32 ). Then,
// implement the following methods for X :
// new : takes a &str and an i32 and returns an X instance
// take_str : takes a mutable reference to self and returns the s field of X , replacing
// it with None


use std::os::macos::raw::stat;
use std::os::unix::raw::ino_t;
use std::path::PathBuf;

struct X{
    s:Option<String>,
    i:i32
}
#[derive(Debug)]

struct User{
  name:String  
}

struct Address <'a>{  // this lifetime specifier makes sure that city variable will be there in memory till the Address object is in memory
    city:& 'a str
}

impl <'a> Address <'a>{
    
}
impl User{
    pub fn update_name(& mut self,name:&str)->&str{
        self.name=name.to_string();
         self.name.as_str()
    }
    
}

impl X{
    
}
fn main() {
    // let  r;
    // {
    //     let x=5;
    //     r=&x;
    //     println!("{}",r);
    // }
   // 
   // // println!("{}",r);
   //  let a=5;
   //  {
   //      let b=59;
   //      let c=part(&a,&b);
   //      println!("{}",c)
   // //  }
   //  let u=yu();  // c will become dangling reference
   //  println!("{}",u)
    
    // let k=hj(&8);
    // println!("{}",k)
    // let new_name="logan";
    // let mut u=User{
    //     name: "john".to_string(),
    // };
    // let o=u.update_name(new_name);
    // println!("{:?}",o)
    
    let a=Address{
        city: "",
    };
}


fn part<'a>(a:&'a i32,b:& 'a i32) ->&'a i32{  // 'a will store the lifetime of shortest of either a variable or b variable
     b
}

// you cannot return a reference from a function
// if you wannt to return a reference that reference should be static
fn yu() -> & 'static i32 {
   let y:& 'static i32=&6;
    y
}


fn hj(u: &i32) -> &i32 {
     u
}

// lifetime ellison rules
// 1- Each parameter that is a reference gets its own distinct lifetime parameter
// 2- IF there is exactly one input lifetime ,its assigned to all output lifetimes
// 3 - if there are multiple input lifetimes ,but one of them is &self or &mut self ,the output lifetime will automatically is that of self

// Generics ---> // Traits --->Trait ---Bounds // dynamic -->dispatch