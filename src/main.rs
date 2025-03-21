

struct Book<'a>{
    title:&'a str
}

fn main() {
    
    let str1=String::from("hello");
    let str2=String::from("there");
    let longest=compare_length(&str1,&str2);
    println!("{}",longest);
    //println!("{}",str1)
}
// Life time is a promise to compiler ,that this reference will live at least as this other one
fn compare_length<'a>(str1:&'a String,str2:&'a String)->&'a String{
    if str1.len()>str2.len(){
      return   str1
    }
     str2
}