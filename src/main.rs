fn main() {
    fn handler(i: &i32) {
        println!("and {i}");
    }

    fn handler2(i: &i32, u: &u8) {
        println!("injection engine says {i} and {u}");
    }

    injection_engine::EngineBuilder::default()
        .with_state(42i32)
        .with_state(7u8)
        .build()
        .run(handler)
        .run(handler2);
}
