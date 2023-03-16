use core::panic;

mod gcloudsecret;
#[tokio::main]
async fn main() {
match gcloudsecret::vault().await {
    Ok(result) => result,
    Err(error) => panic!("Failed to create secret: {}", error),
};
}