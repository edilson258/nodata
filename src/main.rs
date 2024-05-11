mod ast;
mod database;
mod lexer;
mod parser;
mod token;

use database::Table;
use parser::Parser;
use tonic::transport::Server;

use crate::{
    ast::Object,
    database::{Condition, WhereQuery},
};

mod proto {
    tonic::include_proto!("nodata");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("nodata_descriptor");
}

use proto::crud_server::Crud;

struct CrudSerivice {}

impl CrudSerivice {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crud for CrudSerivice {
    fn create_collection<'life0, 'async_trait>(
        &'life0 self,
        request: tonic::Request<proto::CreateCollectionRequest>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<
                    Output = std::result::Result<
                        tonic::Response<proto::CreateCollectionResponse>,
                        tonic::Status,
                    >,
                > + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let res: proto::CreateCollectionResponse = {
            proto::CreateCollectionResponse {
                status: "Done".to_string(),
            }
        };

        println!("Creating {}", request.get_ref().name);

        let fut = async move {
            let response = tonic::Response::new(res);
            Ok(response)
        };

        Box::pin(fut)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = r#"
        "users": {
            "name": String,
            "age": Number,
            "phone": String,
        }
        "#;

    let input = schema.chars().collect::<Vec<char>>();
    let schema = Parser::parse_schema(&input);

    let model1 = r#"
        "users": {
            "name": "Edilson",
            "age": 22,
            "phone": "+55 123-4567",
        }
        "#;

    let model2 = r#"
        "users": {
            "name": "Mungoi",
            "age": 26,
            "phone": "+244 123-4567",
        }
        "#;

    let model3 = r#"
        "users": {
            "name": "Grahms",
            "age": 24,
            "phone": "+34 123-4567",
        }
        "#;

    let input = model1.chars().collect::<Vec<char>>();
    let model1 = Parser::parse_model(&input);

    let input = model2.chars().collect::<Vec<char>>();
    let model2 = Parser::parse_model(&input);

    let input = model3.chars().collect::<Vec<char>>();
    let model3 = Parser::parse_model(&input);

    let mut table = Table::new(schema.unwrap());

    let _ = table.add_row(model1.unwrap());
    let _ = table.add_row(model2.unwrap());
    let _ = table.add_row(model3.unwrap());

    println!(
        "{:#?}",
        table.query_where(WhereQuery {
            field: "name".to_string(),
            cond: Condition::NotEqual(Object::String("Edilson".to_string())),
        })
    );

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    let addr = "0.0.0.0:8000".parse()?;

    Server::builder()
        .add_service(service)
        .add_service(proto::crud_server::CrudServer::new(CrudSerivice::new()))
        .serve(addr)
        .await?;

    Ok(())
}
