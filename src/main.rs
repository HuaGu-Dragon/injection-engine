fn main() {
    fn handler(i: &i32) {
        println!("and {i}");
    }

    let engine = injection_engine::EngineBuilder::default()
        .with_state(42i32)
        .with_state(7u8)
        .build();

    engine.run(handler);
}
