use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

pub mod router;

fn main() {
    println!("Create router");
    let main_router = router::create_test_router(false);
    println!("Router created");
    println!("Match rule");
    let rules = main_router.has_match("https://fr.ouibus.com/fr/montargis".to_string());
    println!("Rules matched {:?}", rules);
}
