use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct User{
     name: String,
     age:i32
}

// impl Debug for User{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

impl User{
    // you can write associated (static functions) and methods
    
    // associated function or static function
  pub fn user_data(name:String,age:i32)->User{
        User{
           name,
            age,
        }
    }
    // methods
    pub fn get_name(&self)->String{  // immutable reference
        self.name.clone()
    }

    pub fn get_age(&self)->i32{ // immutable reference
        self.age
    }
    
    pub fn set_name(&mut self,name:String){  // mutable reference
        self.name=name;
    }
}

// enums  are the custom data types with possible values
#[derive(Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}
// enums with values
#[derive(Debug)]
pub enum Vehicle{
    Car(User),
    Bus(f64),
    Bike(i32),
}

impl Vehicle{
    pub fn vehicle_type(&self){
        match self {
            Vehicle::Car(_) => {}
            Vehicle::Bus(_) => {}
            Vehicle::Bike(_) => {}
        }
        
    }
    
    
}

pub struct Data{
    pub result:Option<String>,
}


// Result enum is way of handling data and error in rust 
// enum Result<T,U>{
//     Ok(T),
//     Err(U)
// }

// enum Option<T>{
//     Some(T),
//     None
// }
