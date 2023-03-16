use core::panic;
use std::fmt::format;

use gcloud_sdk::{
    google::cloud::secretmanager::v1::{
        secret_manager_service_client::SecretManagerServiceClient, CreateSecretRequest,
        ListSecretsRequest, Secret, SecretVersion,
    },
    GoogleApi, GoogleAuthMiddleware, GoogleEnvironment,
};

mod gcloudsecret;
#[tokio::main]
async fn main() {
    let client;
    match gcloudsecret::create_vault_client().await {
        Ok(result) => client = result.clone(),
        Err(error) => panic!("Failed to create gcloud client: {}", error),
    };
    let client2 = client.clone();
    match vault(client).await {
        Ok(result) => result,
        Err(error) => panic!("Failed to list gcloud secrets: {}", error),
    }

    match insert(client2).await {
        Ok(result) => result,
        Err(error) => panic!("Failed to create gcloud secret: {}", error),
    }
}
// lists secrets
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

//insert secret
async fn insert(
    client: GoogleApi<SecretManagerServiceClient<GoogleAuthMiddleware>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let google_project_id: String = GoogleEnvironment::detect_google_project_id().await.expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let response = client
        .get()
        .create_secret(tonic::Request::new(CreateSecretRequest {
            parent: format!("projects/{}", google_project_id),
            secret_id: format!("fdsfadf"),
            secret: std::option::Option::None,
        }))
        .await?;
    println!("Response: {:?}", response);

    Ok(())
}
