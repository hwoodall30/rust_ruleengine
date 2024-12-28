pub mod utils {
    pub mod read_json;
}

use rust_ruleengine::{Condition, FilterFnParams};
use std::time::Instant;
use utils::read_json::read_json_file_streaming;

const ITEMS_PATH: &str = "./tests/items.json";

fn main() {
    let condition = match read_json_file_streaming::<Condition>("./tests/rules/complex.json") {
        Ok(rule) => rule,
        Err(err) => panic!("{}", err),
    };

    let items = match read_json_file_streaming::<Vec<serde_json::Value>>(ITEMS_PATH) {
        Ok(items) => items,
        Err(err) => panic!("{}", err),
    };

    let context =
        match read_json_file_streaming::<serde_json::Value>("./tests/contexts/simple.json") {
            Ok(context) => context,
            Err(err) => panic!("{}", err),
        };

    let filter_params = FilterFnParams {
        items: &items,
        context: &context,
        threshold: None,
    };

    // Warmup runs
    for _ in 0..100 {
        let _ = condition.adaptive_filter(&filter_params);
    }

    let filtered_results = condition.adaptive_filter(&filter_params);
    match filtered_results {
        Ok(items) => println!("Filtered items: {:?}", &items.len()),
        Err(err) => panic!("{}", err),
    }

    // Actual timing
    let iterations = 1000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = condition.adaptive_filter(&filter_params);
    }
    let duration = start.elapsed();
    println!("Average duration: {:?}", duration / iterations as u32);
}
