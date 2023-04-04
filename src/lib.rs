use std::fmt::format;

use anyhow::{Error, Result};

use spin_sdk::{
    config,
    http::{Request, Response},
    http_component,
    mysql::{self, Decode, ParameterValue},
};

// The environment variable set in `spin.toml` that points to the
// address of the MySQL server that the component will write to
const DB_URL_ENV: &str = "DB_URL";

/// A simple Spin HTTP component.
#[http_component]
fn handle_hello_rust(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("GET")
            .uri("http://172.30.1.16") // helloworld running on my k8s cluster
            .body(None)?,
    )?;

    // TODO split into separate routes
    let products = list_products()?;

    let response = format!("Hello!\n\n{:?}\n{:?}", products, res);

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(response.into()))?)
}

fn list_products() -> Result<String, Error> {
    let address = config::get("db_url").expect("Failed to get 'db_url'");
    // let address = std::env::var(DB_URL_ENV)?;

    let sql = "SELECT p.id AS id, p.name AS name, p.image_url AS image_url, c.name AS category_name FROM products p JOIN categories c ON c.id = p.category_id";

    let rowset = mysql::query(&address, sql, &[])?;

    let column_summary = rowset
        .columns
        .iter()
        .map(|column| format!("{}: {:?}", column.name, column.data_type))
        .collect::<Vec<_>>()
        .join(", ");

    let mut products = vec![];

    for row in rowset.rows {
        let product = as_product(&row);
        println!("{:?}", product);
        products.push(format!("{:#?}", product));
    }

    let response = format!(
        "Products:\n{}\n\n(Column info: {})\n",
        products.join("\n"),
        column_summary,
    );

    // Ok(http::Response::builder()
    //     .status(200)
    //     .header("product_count", products.len())
    //     .body(Some(response.into()))?)
    Ok(response)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Product {
    id: i32,
    name: String,
    image_url: String,
    category_name: String,
}

fn as_product(row: &mysql::Row) -> Result<Product> {
    let id = i32::decode(&row[0])?;
    let name = String::decode(&row[1])?;
    let image_url = String::decode(&row[2])?;
    let category_name = String::decode(&row[3])?;

    Ok(Product {
        id,
        name,
        image_url,
        category_name,
    })
}
