use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct User{
     name: String,
     age:i32
}
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
pub enum Weekdays{
    Monday,
    Tuesday
}

impl Weekdays {
    
}