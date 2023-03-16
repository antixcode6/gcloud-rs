use core::panic;

use gcloud_sdk::{
    google::cloud::secretmanager::v1::{
        secret_manager_service_client::SecretManagerServiceClient, ListSecretsRequest,
    },
    GoogleApi, GoogleAuthMiddleware, GoogleEnvironment,
};

mod gcloudsecret;
#[tokio::main]
async fn main() {
    let client;

    match gcloudsecret::vault_client().await {
        Ok(result) => client = result.clone(),
        Err(error) => panic!("Failed to create secret: {}", error),
    };

    match vault(client).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

async fn vault(
    client: GoogleApi<SecretManagerServiceClient<GoogleAuthMiddleware>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let google_project_id = GoogleEnvironment::detect_google_project_id().await
    .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let response = client
        .get()
        .list_secrets(tonic::Request::new(ListSecretsRequest {
            parent: format!("projects/{}", google_project_id),
            ..Default::default()
        }))
        .await?;
    println!("Response: {:?}", response);
    Ok(())
}
