use gcloud_sdk::google::cloud::secretmanager::v1::secret_manager_service_client::SecretManagerServiceClient;
use gcloud_sdk::google::cloud::secretmanager::v1::ListSecretsRequest;
use gcloud_sdk::*;

pub(crate) async fn vault_client(
) -> Result<GoogleApi<SecretManagerServiceClient<GoogleAuthMiddleware>>, Box<dyn std::error::Error>>
{
    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Detect Google project ID using environment variables PROJECT_ID/GCP_PROJECT_ID
    // or GKE metadata server when the app runs inside GKE
    let google_project_id = GoogleEnvironment::detect_google_project_id().await
        .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let secrets_client: GoogleApi<SecretManagerServiceClient<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            SecretManagerServiceClient::new,
            "https://secretmanager.googleapis.com",
            // cloud resource prefix: used only for some of the APIs (such as Firestore)
            None,
        )
        .await?;

    Ok(secrets_client)
}
