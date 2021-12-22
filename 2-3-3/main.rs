struct Apple{}
struct Gold{}
struct FullStomach{}

fn get_gold(a: Apple) -> Gold {
    Gold{}
}

fn get_full_stomach(a: Apple) -> FullStomach {
    FullStomach{}
}

fn my_funcs() {
    let a = Apple{};
    let g = get_gold(a);
    //compile error
    // let s = get_full_stomach(a);
}

fn main(){
    my_funcs()
}