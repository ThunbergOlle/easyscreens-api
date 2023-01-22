use std::{collections::HashMap, string::ParseError};

use aws_sdk_dynamodb::model::AttributeValue;

#[derive(Debug)]
pub struct TV {
    pub id: String,
    
}
impl FromDynamoDB for TV  {
    fn from_dynamodb(value: HashMap<String, AttributeValue>) -> Result<TV, ParseError> {
       Ok(TV {
           id: value.get_string("id").unwrap(),
       })
   }
}

pub trait AttributeValuesExt {
    fn get_string(&self, key: &str) -> Option<String>;
    fn get_number(&self, key: &str) -> Option<u64>;
    fn get_bool(&self, key: &str) -> Option<bool>;
}

impl AttributeValuesExt for HashMap<String, AttributeValue> {
    fn get_number(&self, key: &str) -> Option<u64> {
        self.get(key)?.as_n().ok()?.parse().ok()
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        Some(self.get(key)?.as_bool().ok()?.to_owned())
    }
    fn get_string(&self, key: &str) -> Option<String> {
        Some(self.get(key)?.as_s().ok()?.to_owned())
    }
}
pub trait FromDynamoDB {
    fn from_dynamodb(value: HashMap<String, AttributeValue>) -> Result<Self, ParseError>
    where
        Self: Sized;
}
