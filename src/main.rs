fn main() {
    let string1=String::from("hello");
    let string2=String::from("world");
    let  concatenated_string= concatenated_strings(&string1,&string2);
    print!("result:{}",concatenated_string);
    fn concatenated_strings(st1:&str,st2:&str)->String{
    let mut result=String::new();
    result.push_str(st1);
    result.push_str(st2);
    return result;
    }
    
}
