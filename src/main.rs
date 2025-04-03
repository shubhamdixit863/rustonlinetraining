
//associated types
// trait  Data{
//     type SomeData;   // associated types
//     fn process_data(&self)->Self::SomeData;
// }
//
// struct FileData;
//
//
// impl Data for FileData{
//     type SomeData = i32;
//
//     fn process_data(&self) -> Self::SomeData {
//         return  8;
//     }
//
// }
//
// impl Data for FileData{
//     type SomeData = f64;
//
//     fn process_data(&self) -> Self::SomeData {
//         return  8;
//     }
//
// }



// trait Something <T>{
//     fn foo(&self)->T;
//
// }
//
// struct  Car;
//
// impl Something<i32>  for Car{
//     fn foo(&self) -> i32 {
//        return 90;
//     }
// }
//
// impl Something<f64>  for Car{
//     fn foo(&self) -> i32 {
//         return 90;
//     }
// }
//
//
//
// struct  Van;
// impl Something<f64>  for Van{
//     fn foo(&self) -> f64 {
//          90f64
//     }
// }

use std::rc::Rc;
use std::sync::Arc;

struct  BigData{
    data:[u8;1000000000]
}

struct Node{
    data:i32,
    next:Option<Box<Node>>,
}

struct LinkedList{
    head:Node

}

fn main(){

    // let fd=FileData;
    // println!("{}",fd.process_data());
    // let car=Car;
    // println!("{}",car.foo());
    //
    //  let van=Van;
    // println!("{}",van.foo());

    // Smart pointers in rust //
    // Box<T>

    // let c=9;  // it will be stored in stack
    // println!("{}",c)

    // let c= Box::new(9);  // this will live in heap and c will contain the reference of underlying heap allocation
    // println!("{}",c)

    // let big=Box::new(BigData{
    //     data: [0;1000000000],
    // });

    //   Box pointer
    // when we use recursive data structure

    // let ll=LinkedList{
    //     head: Node{
    //         data: 0,
    //         next: Some(Box::new(Node{ data: 0, next: None })),
    //     },
    // };

    // Rc stands ro Reference counted pointer
    // stored in heap
    //it can have multiple references
    // when the last reference gets dropped the whole data inside the heap gets dropped
    // this is only valid for a particular thread
    // no thread safety
    let a=Rc::new(9);
    let b=Rc::clone(&a);  // it wont create a new memory data and copy all the values instead what it will do ,it will just increment

    // thread safety Arc  // Atomic Reference Count  // thread safe
    // 
    // let a1=Arc::new(9);
    // let b1=Arc::clone(&a1);  // it wont create a new memory data and copy all the values instead what it will do ,it will just increment

}