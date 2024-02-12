use std::{error::Error, io};
mod model_connection;
mod model_enum;
mod model_structure;

use model_enum::MCA_MODEL_ENUM;
use model_connection::*;
use model_structure::*;

fn print_description() {
    println!("-----------------------------");
    println!("r: register model1");
    println!("d: register model2");
    println!("q: quit");
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let _conn = ConnectionBuilder::session()?
    //     .name("org.zbus.DataIPC")?
    //     .serve_at("/org/zbus/DataIPC")?
    //     .build()
    //     .await?;

    // // Do other things or go to wait forever
    // pending::<()>().await;

    loop {
        print_description();
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Fail to read input");

            let trimmed_input = input.trim();

            match trimmed_input.to_lowercase().as_str() {
                "r" => {
                    println!("Register Model1 ");
                    // let mut send_input = String::new();
                    // io::stdin().read_line(&mut send_input)
                    //     .expect("Fail to read input");
                    let mut model_connection_handler = ModelConnection::new();

                    let _result: Result<String, zbus::Error> = model_connection_handler.register_model(MCA_MODEL_ENUM::MODEL_1).await;

                    let _data = IrisData{col1:3.2, col2:3.2 ,col3:3.2 ,col4:3.2};
                    let serialized = serde_json::to_string(&_data).unwrap();
                    
                    let model = model_connection_handler.get_model(MCA_MODEL_ENUM::MODEL_1).unwrap().downcast_ref::<Model1Struct>().unwrap();
                    let _reply = model.test(&serialized.to_string()).await?;
                    println!("Output is {_reply}");

                }
                "d" => {
                    println!("Register Model2 ");
                    // let mut send_input = String::new();
                    // io::stdin().read_line(&mut send_input)
                    //     .expect("Fail to read input");
                    let mut model_connection_handler = ModelConnection::new();

                    let _result: Result<String, zbus::Error> = model_connection_handler.register_model(MCA_MODEL_ENUM::MODEL_2).await;
                    let model = model_connection_handler.get_model(MCA_MODEL_ENUM::MODEL_2).unwrap().downcast_ref::<Model2Struct>().unwrap();
                    let _reply = model.test("testset").await?;

                }
                "q" => {
                    println!("Quit");
                    break;
                }
                _ => {
                    println!("Invalid input");
                }
            }
    }
    Ok(())
}