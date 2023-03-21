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

    /// Value of the secret you create
    #[arg(short, long, default_value = "")]
    value: String,

    /// Profile to use (test, dev, prod)
    #[arg(short, long, default_value = "")]
    profile: String,

    /// list secrets in an account
    #[arg(short, long, default_value_t = false)]
    list: bool,

    // list secrets in an account
    // todo: update secrets in place
    //#[arg(short, long, default_value_t = false)]
    //update: bool,
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
        let client_insert = cli_client.client.clone();
        let client_version = cli_client.client.clone();
        let version_secret = args.secret.clone();

        match vault::insert(client_insert, args.secret).await {
            Ok(result) => result,
            Err(error) => panic!("Failed to create gcloud secret: {}", error),
        }

        match vault::version(client_version, version_secret, args.value).await {
            Ok(result) => result,
            Err(error) => panic!("Failed to version gcloud secret: {}", error),
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
                profile: types::GcloudProfile::Test(String::from("test")),
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
                profile: types::GcloudProfile::Dev(String::from("dev")),
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
                profile: types::GcloudProfile::Prod(String::from("prod")),
                client,
            };
            gcloud_client
        }
        &_ => panic!("unexpected item in bagging area"),
    }
}
