
use std::thread;
use std::sync::Arc;
use std::cell::UnsafeCell;

fn main() {
    let  counter=Arc::new(UnsafeCell::new(0));  // that would be in heap memory
    let mut handles=vec![];
    
    for _ in 0..10{
        let counter_clone=Arc::clone(&counter);
        let handle=thread::spawn(move ||{
            for _ in 0..1000{
                unsafe {
                    *counter_clone.get()+=1;
                }
                
            }
            
        })
        handles.push(handle);
    }

    for handle in  handles {
        handle.join().unwrap();
        
    }
   
    let result=unsafe{
        *counter.get();
        
    };
    println!("Fimnal counter {:?}",result);  // here is different result
}


// use std::thread;
// use std::time::Duration;

// fn main() {
//     // a method which we use to create threads 
//     let h1=thread::spawn(||{
//         // whatever logic we do will happen in a separate thread
//         for i in 1..10{
//             println!("From custom thread - {} ",i)
// 
//         }
// 
//     } );
// 
// 
// 
// 
// 
//     // let c=vec![1,233,4,9];
//     // //let mut j:Vec<i32>=vec![];
//     // // for i in c{
//     // // 
//     // //     j.push(i*2);
//     // //     
//     // // }
//     // // 
//     // // println!("{:?}",j)
//     // let k:Vec<_>=c.iter().map(|x|  x*2).collect();
//     // println!("{:?}",k);
//     for i in 1..10{
//         println!("From main thread - {}",i)
// 
//     }
// 
//     h1.join().unwrap()
//     // let five_seconds = Duration::new(5, 0);
//     thread::sleep(five_seconds)
//     // 
//     in rust we create a thread that is mapped to 1:1 with os threads 
// // }
// // 
