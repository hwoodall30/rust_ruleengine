use std::{collections::HashMap, sync::OnceLock};

use serde_json::Value;

pub type ComparisonFn = fn(&Value, &Value) -> Result<bool, String>;

pub fn get_operator_map() -> &'static HashMap<&'static str, ComparisonFn> {
    static OPERATORS: OnceLock<HashMap<&'static str, ComparisonFn>> = OnceLock::new();
    OPERATORS.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("equals", equals as ComparisonFn);
        m.insert("!equals", not_equals as ComparisonFn);
        m.insert("greater_than", greater_than as ComparisonFn);
        m.insert("less_than", less_than as ComparisonFn);
        m.insert(
            "greater_than_or_equal",
            greater_than_or_equal as ComparisonFn,
        );
        m.insert("less_than_or_equal", less_than_or_equal as ComparisonFn);
        m.insert("contains", contains as ComparisonFn);
        m.insert("not_contains", not_contains as ComparisonFn);
        m
    })
}

#[inline]
pub fn get_operator_fn(operator: &str) -> Option<ComparisonFn> {
    get_operator_map().get(operator).cloned()
}

#[inline]
fn equals(item_value: &Value, value: &Value) -> Result<bool, String> {
    match (item_value, value) {
        (Value::String(item_str), Value::String(val_str)) => Ok(item_str == val_str),
        (Value::Number(item_num), Value::Number(val_num)) => Ok(item_num == val_num),
        (Value::Bool(item_bool), Value::Bool(val_bool)) => Ok(item_bool == val_bool),
        (Value::Array(item_arr), Value::Array(val_arr)) => Ok(item_arr == val_arr),
        (Value::Object(item_obj), Value::Object(val_obj)) => Ok(item_obj == val_obj),
        _ => Ok(false),
    }
}

#[inline]
fn not_equals(item_value: &Value, value: &Value) -> Result<bool, String> {
    equals(item_value, value).map(|result| !result)
}

#[inline]
pub fn greater_than(item_value: &Value, value: &Value) -> Result<bool, String> {
    match (item_value, value) {
        (Value::Number(item_num), Value::Number(val_num)) => {
            let item_f64 = item_num.as_f64().ok_or("Invalid number conversion")?;
            let val_f64 = val_num.as_f64().ok_or("Invalid number conversion")?;
            Ok(item_f64 > val_f64)
        }
        _ => Ok(false),
    }
}

#[inline]
pub fn less_than(item_value: &Value, value: &Value) -> Result<bool, String> {
    match (item_value, value) {
        (Value::Number(item_num), Value::Number(val_num)) => {
            let item_f64 = item_num.as_f64().ok_or("Invalid number conversion")?;
            let val_f64 = val_num.as_f64().ok_or("Invalid number conversion")?;
            Ok(item_f64 < val_f64)
        }
        _ => Ok(false),
    }
}

#[inline]
pub fn greater_than_or_equal(item_value: &Value, value: &Value) -> Result<bool, String> {
    match (item_value, value) {
        (Value::Number(item_num), Value::Number(val_num)) => {
            let item_f64 = item_num.as_f64().ok_or("Invalid number conversion")?;
            let val_f64 = val_num.as_f64().ok_or("Invalid number conversion")?;
            Ok(item_f64 >= val_f64)
        }
        _ => Ok(false),
    }
}

#[inline]
pub fn less_than_or_equal(item_value: &Value, value: &Value) -> Result<bool, String> {
    match (item_value, value) {
        (Value::Number(item_num), Value::Number(val_num)) => {
            let item_f64 = item_num.as_f64().ok_or("Invalid number conversion")?;
            let val_f64 = val_num.as_f64().ok_or("Invalid number conversion")?;
            Ok(item_f64 <= val_f64)
        }
        _ => Ok(false),
    }
}

#[inline]
pub fn contains(item_value: &Value, value: &Value) -> Result<bool, String> {
    match (item_value, value) {
        (Value::String(item_str), Value::String(val_str)) => Ok(item_str.contains(val_str)),
        (Value::Array(item_arr), Value::String(val_str)) => Ok(item_arr
            .iter()
            .any(|v| matches!(v, Value::String(s) if s == val_str))),
        _ => Ok(false),
    }
}

#[inline]
pub fn not_contains(item_value: &Value, value: &Value) -> Result<bool, String> {
    contains(item_value, value).map(|result| !result)
}
