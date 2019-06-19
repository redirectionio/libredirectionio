#[macro_use]
extern crate log;
extern crate stderrlog;

mod router;

use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Api {
    #[serde(rename = "hydra:member")]
    pub rules: Vec<router::rule::Rule>,
}

fn main() -> std::io::Result<()> {
    stderrlog::new().module(module_path!()).timestamp(stderrlog::Timestamp::Second).init().unwrap();

    let mut file = File::open("rules.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    error!("File readed");

    let api: Api = serde_json::from_str(contents.as_str())?;
    error!("Api deserializzd");
    let data = serde_json::to_string(&api.rules)?;
    error!("Rule serializzd");

    let main_router = router::MainRouter::new_from_data(data, false);
    error!("Router created");

    println!("{:?}", main_router);

    Ok(())
}
