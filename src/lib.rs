pub mod operators;
pub mod utils {
    pub mod read_json;
}

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;

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
    pub operator_fn: RefCell<Option<operators::ComparisonFn>>,
}

impl Condition {
    pub fn filter<'a>(
        &self,
        items: &'a [Value],
        context: &Value,
    ) -> Result<Vec<&'a Value>, String> {
        items
            .iter()
            .filter_map(|item| match self.evaluate(&item, &context) {
                Ok(true) => Some(Ok(item)),
                Ok(false) => None,
                Err(e) => Some(Err(e)),
            })
            .collect()
    }

    #[inline]
    fn evaluate(&self, item: &Value, context: &Value) -> Result<bool, String> {
        if let Some(property) = &self.property {
            let item_value_option = item.get(property.as_ref());
            if item_value_option.is_none() {
                return Ok(false);
            }

            let item_value = item_value_option.unwrap();
            if item_value == "*" {
                return Ok(true);
            }

            let func = self.get_operator_func()?;

            if !func(item_value, self.value.as_ref().unwrap())? {
                return Ok(false);
            }
        }

        // Handle sub-conditions if present
        if let Some(sub_conditions) = &self.conditions {
            if !sub_conditions.is_empty() {
                match self.logic {
                    Some(Logic::And) => return self.satisfies_all(&item, &context),
                    Some(Logic::Or) => return self.satisfies_any(&item, &context),
                    None => return self.satisfies_all(&item, &context),
                }
            }
        }

        return Ok(true);
    }

    fn satisfies_all(&self, item: &Value, context: &Value) -> Result<bool, String> {
        self.conditions
            .as_ref()
            .ok_or("Error with conditions")?
            .iter()
            .try_fold(true, |acc, condition| {
                Ok(acc && condition.evaluate(item, context)?)
            })
    }

    fn satisfies_any(&self, item: &Value, context: &Value) -> Result<bool, String> {
        self.conditions
            .as_ref()
            .ok_or("Error with conditions")?
            .iter()
            .try_fold(false, |acc, condition| {
                Ok(acc || condition.evaluate(item, context)?)
            })
    }

    #[inline]
    fn get_operator_func(&self) -> Result<operators::ComparisonFn, String> {
        if self.operator_fn.borrow().is_none() {
            let operator = self.operator.as_ref().ok_or("Operator is required")?;
            let func = operators::get_operator_fn(operator).ok_or("Operator not found")?;
            *self.operator_fn.borrow_mut() = Some(func);
        }
        Ok(self.operator_fn.borrow().unwrap())
    }
}
