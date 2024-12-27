pub mod utils {
    pub mod read_json;
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use crate::utils::read_json::read_json_file_streaming;
    use rust_ruleengine::Condition;

    const ITEMS_PATH: &str = "./tests/items.json";

    #[test]
    fn filter_simple_test() {
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

        let start = Instant::now();
        let result = condition.filter(&items, &context);
        let duration = start.elapsed();
        println!("Duration: {:?}", duration);
        match result {
            Ok(result) => {
                assert_eq!(result.len(), 1466)
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}
