struct Foo {
    val: u32
}

fn add<'a>(x: &'a Foo, y: &'a Foo) -> u32 {
    x.val + y.val
}

fn my_func6(){
    let x = Foo{val: 10};
    {
        let y = Foo{val: 20};
        let z = add(&x, &y);
        println!("z = {}", z);
    }
}

fn main(){
    my_func6();
}