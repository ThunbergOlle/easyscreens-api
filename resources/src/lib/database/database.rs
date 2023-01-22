#[path = "../../lib/database/models.rs"]
pub(crate) mod models;
use std::fmt::Display;

use aws_sdk_dynamodb::{
    error::{GetItemError},
    model::AttributeValue,
    types::SdkError,
    Client, Error as DynamoDBError,
};

use aws_sdk_ssm as ssm;

pub struct DBClient {
    pub client: Client,
    pub table_name: String,
}
pub enum SetupError {
    DynamoDBError(DynamoDBError),
    SSMError(ssm::Error),
}

impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::DynamoDBError(error) => write!(f, "DynamoDBError: {}", error),
            SetupError::SSMError(error) => write!(f, "SSMError: {}", error),
        }
    }
}
pub enum QueryError {
    GetItemError(SdkError<GetItemError>),
    NotFoundError,
}
impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::GetItemError(error) => write!(f, "GetItemError: {}", error),
            QueryError::NotFoundError => write!(f, "NotFoundError"),
        }
    }
}
impl DBClient {
    pub async fn get_item<DocumentStruct: models::FromDynamoDB>(
        &self,
        key: &str,
    ) -> Result<DocumentStruct, QueryError> {
        let value = AttributeValue::S(key.to_string());
        let get_item_request = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("id", value)
            .send()
            .await;

        match get_item_request {
            Ok(output) => {
                if output.item.is_none() {
                    log::info!("Item not found");
                    return Err(QueryError::NotFoundError);
                }
                log::info!("Item: {:?}", output.item);
                let item = DocumentStruct::from_dynamodb(output.item.unwrap());
                Ok(item.unwrap())
            }
            Err(error) => {
                log::error!("Error: {}", error);
                Err(QueryError::GetItemError(error))
            }
        }
    }
}

pub async fn setup(table_name: &str) -> Result<DBClient, SetupError> {
    let config = aws_config::load_from_env().await;

    log::info!("ssm_config: {:?}", config);
    let ssm_client = ssm::Client::new(&config);

    let table_name_request = ssm_client
        .get_parameter()
        .name(&format!("/tables/{}", table_name))
        .send()
        .await;

    let table_cdk_name;
    match table_name_request {
        Ok(output) => {
            table_cdk_name = output.parameter.unwrap().value.unwrap();
            log::info!("tableARN: {}", table_cdk_name);
        }
        Err(error) => {
            log::error!("Error: {}", error);
            return Err(SetupError::SSMError(error.into()));
        }
    }

    let client = Client::new(&config);
    // check if we have permissions to the table
    let describe_table = client
        .describe_table()
        .table_name(&table_cdk_name)
        .send()
        .await;

    match describe_table {
        Ok(output) => {
            log::info!("Table: {:?}", output.table);
        }
        Err(error) => {
            log::error!("Error: {}", error);
            return Err(SetupError::DynamoDBError(error.into()));
        }
    }
    let db_client = DBClient {
        client: client,
        table_name: table_cdk_name,
    };
    Ok(db_client)
}
