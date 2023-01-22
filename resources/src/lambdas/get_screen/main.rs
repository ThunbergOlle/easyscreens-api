#[path = "../../lib/database/database.rs"]
mod database;


use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use serde_json::{json, Value};
use simple_logger::SimpleLogger;



#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = service_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Value> {
    let table_name = "screens";

    let db: database::DBClient;
    match database::setup(table_name).await {
        Ok(client) => {
            log::info!("Using table: {}", client.table_name);
            db = client;
        }
        Err(error) => {
            log::info!("Error: {}", error);
            return Err(json!({"error":"There was an error trying to setup the lambda function"}));
        }
    }
    
    let (event, _) = event.into_parts();
    let tv_id = event["tv"]
        .as_str()
        .ok_or(json!({"error":"Invalid input"}))?;
    
        // db get item
    let tv: database::models::TV;
    match db.get_item(tv_id).await {
        Ok(item) => {
            tv = item;
            log::info!("Item: {:?} was found!", &tv.id);
        }
        Err(error) => {
            log::error!("Error: {}", error);
            return Err(json!({"error":"There was an error trying to get the item"}));
        }
    }
  
    Ok(json!({ "response": "OK" }))
}
