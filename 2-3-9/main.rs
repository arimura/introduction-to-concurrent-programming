use std::thread::spawn;

fn hello(){
    println!("Hello World!");
}

fn my_function(){
    spawn(hello).join();

    let h = || println!("Hello World!");
    spawn(hello).join();
}

fn my_func12(){
    let v = 10;
    let f = move || v * 2;

    let result = spawn(f).join();
    println!("result = {:?}", result);

    match spawn(|| panic!("I'm paniced!")).join() {
        Ok(_) => {
            println!("succeeded");
        }
        Err(a) => {
            let s = a.downcast_ref::<&str>();
            println!("failed: {:?}", s);
        }
    }
}
fn main(){
    // my_function();
    my_func12(); 
}