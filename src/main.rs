mod service;
mod controllers;

use service::kafkaservice::add;

use std::fs;
use std::mem::{drop, forget};
use serde_json;
// fn name(num:i8)->i8{
//      2*num
// }
//
// fn name2(num:i8){
//
// }


// fn is_true(num :i8)->bool{
//     if num >7{
//         true
//     }else if num >5{
//          false
//     }else {
//         true
//     }
// 
// }

fn Multiply(num :i8)->i8{
    2*num
    
}


// fn ConvertString(name :String){
//    println!("{:?}",name)
// 
// }


fn ConvertStringBorrow(name :&mut String){
  //  println!("{:?}",name)
   name.push('d')
}

#[derive(Debug)]
struct  Resource{
    name:String
}

// impl Drop for Resource{
//     fn drop(&mut self) {
//         // file system  --->they would be doing some clean up process or may the would be calling some syscall
//         println!("{}","clean up gets called")
//     }
// }

// mod a {
//     pub fn foo() {}
// }
// mod b {
//     pub fn foo() {
//         super::a::foo();  // here super represents main.rs
//     }
// }

fn main() {
  //  use internal;

    // datatypes

    // // Integer types --->i8 ,i16,i32 ,i64,i128, u8 ,u16 ,u32 ,u64,u128
    //
    // let mut  num :i8=9; // rust doest have concept of garbage value or zero value (go)
    //
    // // float data types  -- f32 ,64
    //
    // let f:f32=9.9;
    //
    // //  bool
    //
    // let isActive:bool =true;
    //
    // // character data type
    //
    // let c:char='A';
    //
    // // Strings &str
    //
    //
    // // compound data types
    //
    // // tuples ,fixed size collections
    // let   person=(20,20,20);
    //
    // // arrays
    //
    // //let num=[10,20,30];
    //
    // //let c:[i32;3]=[10,23,90];
    //
    //
    // let o=name(3);
    //
    // println!("{}",o)

  // let y=  is_true(8);
  //   println!("{}",y)
  //   let condition =false;
  //   let value= if condition{ 9} else if !condition {99 } else {90};  // expression
  //   println!("{}",value);
  //
  //   let  mut count=0;

    // loops   ,loop keyword infinite loop ,while ,for 
    // loop {
    //     println!("{}",9);
    //     if count >100{
    //         break;
    //     }
    //     count=count+1;
    // }

    // while count<9 {
    //     println!("{}","counted");
    //     
    //     count=count+1;
    //     
    // }
    
   // 0 to 10  for i=0;i<10;i++
   //
   //  for i in 0..10{
   //      println!("{}",i)
   //  }
   //

    // let v=vec![1,2,3,3,];
    // for i in v.iter(){
    //     println!("{}",i)
    //
    // }
    // let c :Vec<i32> =v.iter().map(|x|x*2).collect();  // functional programming
    //
    // println!("{:?}",c)

    // Ownership and borrowing
    // Drop semantics
    // borrow checker
    // Each value in rust has a single owner
    // if you change ownership --->the  ownership either gets moved ,cloned or copied
    // When the owner goes out of scope the value is dropped

    // let c=9;
    // let d=c;
    //
    // println!("{}",c);
    //
    // println!("{}",d)
    // // copy  , dealing any primitive types
    // &str ,String
    
    // Copy Trait ,// Move Trait // Clone Trait
    
    // move is happening
    // let  str=String::from("HEllo");
    // 
    // let d=str;  // the ownership of hello is being moved from str to d
    // 
    // //println!("{}",d);
    // 
    // // what if you want both str and d to be valid
    // // you can do cloning
    // 
    // let  str1=String::from("HEllo world");
  
    // let d1=str1.clone();  
    // println!("{}",d1);
    // println!("{}",str1)
    
    // let y=9;
    // let result=Multiply(y);
    // println!("{}",result)
    // 
    // let h=String::from("hello");
    // 
    // ConvertString(h);
    // println!("{}",h)
    
    // ---- Borrowing
    // 
     //let mut s=String::from("hello");
    //  let mut c=&mut s;
    // let mut s1=String::from("helloegegge");
    // 
    // c=&mut s1;
    // // 
    // println!("{:?}",c);
    // 
    // ConvertStringBorrow(c);
    // println!("{:?}",s);

    // ConvertStringBorrow(c);
    // 
    // println!("{}",s)
    // let c1=&s;
    // let c2=&s;
    // println!("{},{}",c1,c2)
    // let v=&mut s;

    // let y=&s;

    // println!("{}",v);  // Display trait ,Debug trait
    
    // 1- You can have multiple immutable references of the data
    
    // 2- you can have only mutable borrow /reference
    // 3 - immutable and mutable reference cant co exist
    
    //  Drop semantics 
    
    // Automatically drop the variable from memory once its out of scope or its moved
    // let st=Resource{  // this struct gets memory in the heap
    //     name:String::from("hello ")
    // };
    
    // println!("{},{:?}","resource created",st);
    
    //drop(st)  // std mem  it automatically gets called
    
    // When the Drop is called 
    
    // 1- Variable goes out of scope
    // 2- Ownership move to another variable
    // 3- Ownership transferred to another function args
    // 4- Manually just call drop(
    
    // let o=String::from("hello");
    // let c=o;  // rust will drop o ,it will pop it out of stock
    // println!("{}",o);
    
    // Preventing drop  std::mem::forget
      
    // never do this 
    // forget(st) // Prevent drop  // this is memory leak
    // 
    // 
    
    
    // We will talk about vectors
    // different ways to create a vector 
   // Vec::new()
    
    // vector is a descriptor of underlying array that is created on the heap memory
    
    
    // let mut a:Vec<i32>=Vec::with_capacity(8);  // this will create a vector with length 0 and capacity 0
    // 
    // println!("{}",a.len());
    //  println!("{}",a.capacity()) ; // reserved value
    // let mut b=vec![];
    // println!("{:?}",b);
    // b.push(9);
    
    // println!("{}",b.len());
    // println!("{}",b.capacity()) ; // reserved value
    // 
    // b.push(91);
    // println!("{}",b.len());
    // println!("{}",b.capacity()) ; // reserved value
    // 
    // b.push(911);
    // println!("{}",b.len());
    // println!("{}",b.capacity()) ; // reserved value
    // 
    // b.push(789);
    // println!("{}",b.len());
    // println!("{}",b.capacity()) ; // reserved value
    // 
    // b.push(7899);
    // println!("{}",b.len());
    // println!("{}",b.capacity()) ; // reserved value
    
    // capacity is just a reserved value which when filled makes the compiler re allocate the whole underlying array
    
    // how to iterate over vectors 
    
    // for loop ,nd some  functional programming
    
    // it returns immutable references
    // for i in b.iter(){
    //     println!("{}",i)
    //     
    // }

    
    // If we want to modify the underlying vector elements
    // println!("{:?}",b);
    // // it returns mutable reference
    // for i in b.iter_mut(){
    //     // We have to perform de reference  *pointer
    //    *i=*i*2;  
    //     
    // }
    // 
    // println!("{:?}",b)
    
    // if we want to get the ownership of underlying elements
    
    // for i in b.into_iter(){
    //     println!("{}",i);
    //     a.push(i)
    // }
  //  println!("{:?}",b) wont work

    // println!("{:?}",a)

    // We will talk about closures
    // closures are function ,but what happens with them is they capture their surrounding environment
    // |args| {body}
    // ----------immutable rference
    // let name="Rust";
    // 
    // let add= |x1,y1| {
    //     // logic here 
    //     println!("{}",name);
    //     x1+y1
    // };  // closure are define
    // 
    // println!("{}",add(7,8))
    
    // capturing the environment variable using a mutable reference 
    // 
    // let mut count=0;
    // 
    // let mut increment=||{
    //     count+=1;  // reference of environment variable count
    //     println!("{}",count)
    //     
    // };
    // 
    // increment();
    // println!("{}",count) ; // we can still access count here that means the count is passed as reference
    // 
    // 
    // if we want to pass the ownership to the closure
    // move keyword
    
   // let  mut str= String::from("hello");
    // 
    // let mut modify=|| {
    //       str.push('h')  // we got a mutable reference
    // };
    // 
    // modify();
    // println!("{}",str)  // we borrowed the string 
    
    // we will take ownership of the str
    // let mut own=move ||  {
    //     str.push('j');
    // };
    // own();
    
    //println!("{}",str)  // it wont work because we are using the moved value 
    // Rust closure implements some trait 
    // Fn -- this trait is implemented by the closure whenever you are using the env variable as immutable reference
    //FnMut  -- this trait is implemented by the closure whenever you are using the env variable as mutable reference
    //FnOnce -this trait is implemented by the closure whenever you are using the env variable by taking ownership
    
    // Use of closure with vectors functional programming
    
   //  let vb=vec![1,2,-2,3,3,4,-1,-90];
   //  // filter
   //  // map 
   //  // sum
   //  
   //  // we want to filter out all the elements greater than 0
   //  // let y:Vec<&i32>=vb.iter().filter(|x|**x>0).collect() ;// collect will store into another vector
   //  // 
   //  // println!("{:?}",y);
   // 
   //  //let y:i32=vb.iter().filter(|x|**x>0).sum() ;// collect will store into another vector
   //  let z:Vec<i32>=vb.iter().map(|x| *x*2).collect();
   // println!("{:?}",z);
    
    //https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    // ---->  try reading a json file //  [1,2,3,3,4,45]
    // convert the data into vector 
    // then filter the values that are negative
    // write the data in a new file

    // let contents = fs::read_to_string("./data.json") // Vec<u8>
    //     .expect("Should have been able to read the file");
    //
    // let vector_data:Vec<i8>=serde_json::from_str(&contents).unwrap();  // deserialization
    //
    //
    // let filtered_data:Vec<i8>=vector_data.iter().filter(|x|**x>0).map(|x|*x).collect();
    // //let y=Vec::from(contents);
    // println!("{:?}",filtered_data);
    // println!("{:?}",vector_data[0]);

    //  -->write the data in the file

    // We have to convert this string into a vector

    // deserialization

    // let c=add(&9,&9);
    // println!("{}",c)


    controllers::user_controller::publish_kafka_message()


}
