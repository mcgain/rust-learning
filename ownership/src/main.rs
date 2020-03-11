fn main() {
    // let x = 1;
    let s = String::from("foo");

    take_ownership(s);

    // println!("x is {} and s is {}", x, s); // won't work because s has gone

    let t = give_me_a_string();
    println!("t = {}", t);

    let u = give_and_return_a_string(t);
    println!("after the return = {}", u);

    let v = String::from("foo");
    let len = get_length(&v);
    println!("after the return = {} is {} long", v, len);

    let mut w = String::from("foo");
    add_some_stuff(&mut w);
    add_some_more(&mut w);
    println!("w is now: {}", w);

}

fn add_some_more(w: &mut String) {
    w.push_str(" more");
}

fn add_some_stuff(w: &mut String) {
    w.push_str(" and another thing . . .");
}


fn get_length(v: &String) -> usize {
    v.len()
}

fn give_and_return_a_string(mut t: String) -> String {
    t.push_str(" baz");
    t
}

fn give_me_a_string() -> String {
    String::from("bar")
}

fn take_ownership(s: String) {
    println!("inside ownership s = {}", s);
}
