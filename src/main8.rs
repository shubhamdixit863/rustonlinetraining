// To do app
// we will take the the task from the user
// we will store that in memory
// we will then print it out as well

use std::fmt::Debug;
use std::io;
use std::io::Write;

#[derive(Clone,Debug)]
enum Priority {
    Low,
    Medium,
    High,
}



trait Task {
    fn get_priority(&self) -> Priority;  // method
    fn get_title(&self) -> String;
}
#[derive(Clone,Debug)]
enum Status{
    Completed,
    Pending,
    Aborted

}

#[derive(Debug)]
struct SimpleTask{
    title:String,
    priority:Priority,
    status:Status
}

// let y=get_title()
impl Task for SimpleTask{
    fn get_priority(&self) -> Priority{  //return  borrowed values
        self.priority.clone()

    }

    fn get_title(&self) -> String{  // returning an owned value return the &str borrowed values
        self.title.clone()
    }


}


// taking the task input from user
// Your task is try returning the Error if the user enter any other valeu for priority
fn get_task_input()->Result<SimpleTask,String>{
    // users input
    print!("Enter task title:");
    io::stdout().flush().unwrap();
    let mut title=String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");
    print!("Enter task priority:");
    io::stdout().flush().unwrap();
 
    let mut priority=String::new();
    let mut priority_val:Priority = Priority::Low;

    io::stdin().read_line(&mut priority).expect("Failed to read line");
    io::stdout().flush().unwrap();
    match priority.trim() {
        "low"=>priority_val=Priority::Low,
        "medium"=>priority_val=Priority::Medium,
        "high"=>priority_val=Priority::High,
        _=> println!("Unknown value")
        
    }
    
    let mut status=Status::Pending;
    
    let task=SimpleTask{
        title,
        priority: priority_val,
        status,
    };
    
    Ok(task)

}

fn print_task<T>(task:T) where T: Task+Debug{  // trait bounds
    println!("{:?}",task)
}

struct Data{
    status:Status,
    data:String,
    
}
fn main(){
    let task=get_task_input();
    
   // we will extract the Task struct from the Result enum above
    
    // match task{
    //     Ok(taskValue) => {
    //         print_task(taskValue);
    //     }
    //     Err(_) => {
    //         println!("Error reading input")
    //     }
    // }
    print_task(task.unwrap());
    
}

