use redirectionio::update_rules_for_router;

fn main() {
    std::thread::Builder::new().stack_size(50_000).spawn(|| {
        update_rules_for_router("test".to_string(), "[]".to_string(), 1000);
    }).unwrap().join().unwrap()
}

