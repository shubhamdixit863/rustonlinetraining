struct Student {
    name:String,
    
}

impl Student{
    fn new(name:&str)->Student{  // static 
        Self{
            name:name.to_string(),
        }
    }
    
    fn get_name(&self) ->&str{  // method 
        &self.name
    }
    
    
}


// traits  

trait Studious{
    fn study_hard(&self) ->bool;
}

impl Studious for Student{
    fn study_hard(&self) -> bool {
        true
    }
}

// enun 
enum Weekdays{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}




// Result and Option Enuns


fn main() {
    
    // let c =9;
    // let f=c;
    // 
    // println!("{}",f);
    // println!("{}",c);
    // 
    
    // let g= String::from("hello"); // heap
    // let h=g;
    // println!("{}",g);  // g gets dropped
    // println!("{}",h);
    
    // copy 
    // move 
    // 
    // let g= String::from("hello"); // heap
    // let h=g.clone();
    // println!("{}",g);  // g gets dropped
    // println!("{}",h);
    
    
    //  borrowing 
    // 
    // let g= String::from("hello"); // heap
    // let h=&g;
    // 
    // println!("{}",h);
    // println!("{}",g);
    
    // borrowing rules
    // you can have n number of  non-mutable  references
    // you can have only a single mutable reference
    // you cannot have mutable and immutable reference exist together 
    
    let mut  k=vec![1,2,3,43];
    let  h=& k;
    let i=& k;
    
    // h.push(1);
    // println!("{:?}", h);
    
  //  let c:bool;
    
    let st= Student::new("S");
    
    st.get_name();
    st.study_hard();
    
    let w=Weekdays::Thursday;
    
    
}

fn foo() ->String{
    
    return "hello".to_string();
    
}

fn Add()->Option<String>{
    None
    
}

fn divide(number1:i32, number2:i32)->Result<i32,String>{
    if number1==0{
       return  Err("Cannot divide by 0".to_string())
    }
    let v=number1/number2;
    Ok(v)
    
}

// generic 
// Lifetimes
// Will start with project building