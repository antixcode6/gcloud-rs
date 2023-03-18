use clap::Parser;
use core::panic;
use types::GcloudClient;
mod gcloudsecret;
mod types;
mod vault;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the secret to put into gcloud
    #[arg(short, long, default_value = "")]
    secret: String,

    /// Profile to use (test, dev, prod)
    #[arg(short, long, default_value = "")]
    profile: String,

    /// list secrets in an account
    #[arg(short, long, default_value_t = false)]
    list: bool,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    //let client;
    let cli_client = build_client(args.profile).await;

    if args.list {
        let list_secrets = cli_client.client.clone();

        match vault::list_vault(list_secrets).await {
            Ok(result) => result,
            Err(error) => panic!("Failed to list gcloud secret vault values: {}", error),
        }
    }

    if args.secret != "" {
        let client2 = cli_client.client.clone();

        match vault::insert(client2, args.secret).await {
            Ok(result) => result,
            Err(error) => panic!("Failed to create gcloud secret: {}", error),
        }
    }
}

// profile name doesn't work currently
async fn build_client(profile: String) -> GcloudClient {
    let client;

    match profile.as_str() {
        "test" => {
            match gcloudsecret::create_vault_client().await {
                Ok(result) => client = result,
                Err(error) => panic!("Failed to create gcloud client: {}", error),
            };
            let gcloud_client = GcloudClient {
                profile: types::GcloudProfile::Test(String::from("cyderes-test")),
                client,
            };
            gcloud_client
        }
        "dev" => {
            match gcloudsecret::create_vault_client().await {
                Ok(result) => client = result,
                Err(error) => panic!("Failed to create gcloud client: {}", error),
            };
            let gcloud_client = GcloudClient {
                profile: types::GcloudProfile::Dev(String::from("cyderes-dev")),
                client,
            };
            gcloud_client
        }
        "prod" => {
            match gcloudsecret::create_vault_client().await {
                Ok(result) => client = result,
                Err(error) => panic!("Failed to create gcloud client: {}", error),
            };
            let gcloud_client = GcloudClient {
                profile: types::GcloudProfile::Prod(String::from("cyderes-prod")),
                client,
            };
            gcloud_client
        }
        &_ => panic!("unexpected item in bagging area"),
    }
}
