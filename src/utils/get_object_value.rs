use serde_json::Value;

#[inline]
pub fn get_object_value<'a>(object: &'a Value, key: &str) -> Result<&'a Value, String> {
    match key
        .split('.')
        .try_fold(object, |acc, key| match acc.get(key) {
            Some(value) => Ok(value),
            None => Err(format!("Key '{}' not found", key)),
        }) {
        Ok(value) => Ok(value),
        Err(e) => Err(e),
    }
}
