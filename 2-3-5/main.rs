struct Foo {
    val: u32
}

fn add_val(x: Foo, y: Foo) -> (u32, Foo, Foo){
    (x.val + y.val, x, y)
}

fn mul_val(x: Foo, y: Foo) -> (u32, Foo, Foo) {
    (x.val * y.val, x, y)
}

fn my_func7(){
    let x = Foo{val: 3};
    let y = Foo{val: 6};
    let (a, xn, yn) = add_val(x,y);
    let (b, _, _) = mul_val(xn, yn);
    println!("a = {}, b = {}", a, b);
}

fn my_func8(){
    let mut x = Foo{val: 10};
    {
        let a = &mut x;
        println!("a.val = {}", a.val);

        // println!("x.val = {}", x.val);
        let b: &Foo = a;
        // a.val = 20;
        println!("b.val = {}",b.val);
        a.val = 30;
    }

    {
        let c = &x;
        println!("c.val = {}",c.val);
        println!("x.val = {}",x.val);

        let d = &mut x;
        d.val = 40;

        // println!("c.val = {}", c.val);
    }
}

fn main(){
    // my_func7();
    my_func8();
}