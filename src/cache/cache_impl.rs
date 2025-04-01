use std::fmt::Debug;

pub trait  EvictionPolicy{
    

    fn evict(&self);
}


#[derive(Debug)]
pub struct LRU<T:Debug>{
    data:T

}

impl<T:Debug> LRU<T>{

    pub fn new(data:T)-> Box<LRU<T>> { 
      Box::new(   LRU{
            data,
        })
     
    }

}

impl <T:Debug> EvictionPolicy for LRU<T>{
    fn evict(&self) {
        println!("LRU GETS CALLED ,AND WE ARE EVICTIN THE DATA:{:?}",self.data)
    }
}
#[derive(Debug)]
pub struct LFU <T>{
    data:T
}

 impl<T:Debug> LFU<T>{

     pub fn new(data:T)-> Box<LFU<T>> {
         Box::new(   LFU{
             data,
         })

     }

}


impl <T:Debug> EvictionPolicy for LFU<T>{
    fn evict(&self) {
        println!("LFU GETS CALLED ,AND WE ARE EVICTIN THE DATA:{:?}",self.data)

    }
}