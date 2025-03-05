

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


fn main() {
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


}
