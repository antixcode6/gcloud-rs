use clap::Parser;
use core::panic;
mod gcloudsecret;
mod vault;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the secret to put into gcloud
    #[arg(short, long)]
    secret: String,

    /// Profile to use (test, dev, prod)
    #[arg(short, long)]
    profile: String,

    /// list secrets in an account
    #[arg(short, long)]
    list: bool,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client;

    match gcloudsecret::create_vault_client().await {
        Ok(result) => client = result.clone(),
        Err(error) => panic!("Failed to create gcloud client: {}", error),
    };
    if args.list {
        let list_secrets = client.clone();

        match vault::list_vault(list_secrets).await {
            Ok(result) => result,
            Err(error) => panic!("Failed to list gcloud secret vault values: {}", error),
        }
        return;
    }

    let client2 = client.clone();

    match vault::insert(client2).await {
        Ok(result) => result,
        Err(error) => panic!("Failed to create gcloud secret: {}", error),
    }
}
