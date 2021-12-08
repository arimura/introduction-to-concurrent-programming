enum Gender {
    Male,
    Female
}

enum Role {
    Player(u32, u64),
    Supporter(u32)
}

struct Person{
    age: u32,
    gender: Gender,
    role: Role
}

enum Option<T> {
    Some(T),
    None
}

struct Pair<A> {
    first: A,
    second: A
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn main() {
    // 世界よ、こんにちは
    println!("Hello, world!");
    Person {
        age: 20,
        gender: Gender::Female,
        role: Role::Supporter(70)
    };
}
