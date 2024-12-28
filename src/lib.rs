pub mod operators;
pub mod utils {
    pub mod read_json;
}

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::RwLock;

#[derive(Debug, Deserialize, Serialize)]
pub enum Logic {
    And,
    Or,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Condition {
    pub operator: Option<Box<str>>,
    pub property: Option<Box<str>>,
    pub value: Option<Box<Value>>,
    pub variable: Option<Box<str>>,
    pub logic: Option<Logic>,
    pub conditions: Option<Box<[Condition]>>,

    #[serde(skip)]
    operator_fn: RwLock<Option<operators::ComparisonFn>>,
}

pub struct FilterFnParams<'a> {
    pub items: &'a [Value],
    pub context: &'a Value,
    pub threshold: Option<usize>,
}

impl Condition {
    pub fn filter<'a>(&self, params: &'a FilterFnParams) -> Result<Vec<&'a Value>, String> {
        params
            .items
            .iter()
            .filter_map(|item| match self.evaluate(item, params.context) {
                Ok(true) => Some(Ok(item)),
                Ok(false) => None,
                Err(e) => Some(Err(e)),
            })
            .collect()
    }

    pub fn parallel_filter<'a>(
        &self,
        params: &'a FilterFnParams,
    ) -> Result<Vec<&'a Value>, String> {
        params
            .items
            .par_iter()
            .filter_map(|item| match self.evaluate(item, params.context) {
                Ok(true) => Some(Ok(item)),
                Ok(false) => None,
                Err(e) => Some(Err(e)),
            })
            .collect()
    }

    pub fn adaptive_filter<'a>(
        &self,
        params: &'a FilterFnParams,
    ) -> Result<Vec<&'a Value>, String> {
        if params.items.len() <= params.threshold.unwrap_or(20000) {
            return self.filter(params);
        } else {
            return self.parallel_filter(params);
        }
    }

    #[inline]
    fn evaluate(&self, item: &Value, context: &Value) -> Result<bool, String> {
        if let Some(property) = &self.property {
            let comparison_value = if let Some(variable) = &self.variable {
                context
                    .get(variable.as_ref())
                    .ok_or("Error getting context value")?
            } else {
                self.value.as_ref().ok_or("Error getting value")?
            };

            if comparison_value == "*" {
                return Ok(true);
            }

            let item_value_option = item.get(property.as_ref());
            if item_value_option.is_none() {
                return Ok(false);
            }

            let item_value = item_value_option.ok_or("Error getting item value")?;

            let func = self.get_cached_operator_fn()?;

            if !func(item_value, comparison_value)? {
                return Ok(false);
            }
        }

        if let Some(sub_conditions) = &self.conditions {
            if !sub_conditions.is_empty() {
                match self.logic {
                    Some(Logic::And) => return self.satisfies_all(item, context),
                    Some(Logic::Or) => return self.satisfies_any(item, context),
                    None => return self.satisfies_all(item, context),
                }
            }
        }

        return Ok(true);
    }

    #[inline]
    fn satisfies_all(&self, item: &Value, context: &Value) -> Result<bool, String> {
        let conditions = self.conditions.as_ref().ok_or("Error with conditions")?;

        for condition in conditions.iter() {
            if !condition.evaluate(item, context)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    #[inline]
    fn satisfies_any(&self, item: &Value, context: &Value) -> Result<bool, String> {
        let conditions = self.conditions.as_ref().ok_or("Error with conditions")?;

        for condition in conditions.iter() {
            if condition.evaluate(item, context)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[inline]
    fn get_cached_operator_fn(&self) -> Result<operators::ComparisonFn, String> {
        {
            let guard = self.operator_fn.read().map_err(|_| "RwLock poisoned")?;
            if let Some(func) = *guard {
                return Ok(func);
            }
        }

        let mut guard = self.operator_fn.write().map_err(|_| "RwLock poisoned")?;
        if guard.is_none() {
            let operator = self.operator.as_ref().ok_or("Operator is required")?;
            let func = operators::get_operator_fn(operator).ok_or("Operator not found")?;
            *guard = Some(func);
        }

        Ok(guard.as_ref().unwrap().clone())
    }
}
