#![allow(warnings)]
fn main(){
    let mut mood = String::from("happy");
    // borrow_ref(&mood);
    let a = &mood;
    mutate_string(&mut mood);
    borrow_ref(&mood);
    println!("{},{}",mood,a);
}


fn mutate_string(s:&mut String) {
    s.push_str("hello");
}
fn borrow_ref(s:& String) {
   println!("[borrowed] {}",s);
}
