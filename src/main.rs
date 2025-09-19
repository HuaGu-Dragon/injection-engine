fn main() {
    fn handler(i: &i32) {
        println!("and {i}");
    }

    fn handler2(i: &i32, u: &u8) {
        println!("injection engine says {i} and {u}");
    }

    fn handler3(i: &i32, u: &u8, s: &&str, v: &Vec<i32>) {
        println!("injection engine says {i}, {u}, {s}, and {v:?}");
    }

    let vec = vec![1, 2, 3];
    injection_engine::EngineBuilder::default()
        .with_state(42i32)
        .with_state(7u8)
        .with_state("Hello, world!")
        .with_state(vec)
        .build()
        .run(handler)
        .run(handler2)
        .run(handler3);
}
