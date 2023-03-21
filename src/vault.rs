use gcloud_sdk::{
    google::cloud::secretmanager::v1::{
        replication::Automatic, secret_manager_service_client::SecretManagerServiceClient,
        AddSecretVersionRequest, CreateSecretRequest, ListSecretsRequest, Replication, Secret,
    },
    proto_ext::secretmanager::SecretPayload,
    GoogleApi, GoogleAuthMiddleware, GoogleEnvironment,
};
use std::collections::HashMap;

// lists secrets
pub async fn list_vault(
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
    println!("Response: {:#?}", response);
    Ok(())
}

//insert secret
pub async fn insert(
    client: GoogleApi<SecretManagerServiceClient<GoogleAuthMiddleware>>,
    secret_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let google_project_id: String = GoogleEnvironment::detect_google_project_id().await.expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let secret_insertion = secret_name.clone();
    // this fucking sucks
    let secrets = Some(Secret {
        name: secret_name,
        replication: Some(Replication {
            replication: Option::Some(
                gcloud_sdk::google::cloud::secretmanager::v1::replication::Replication::Automatic(
                    Automatic {
                        customer_managed_encryption: None,
                    },
                ),
            ),
        }),
        create_time: None,
        etag: "".to_string(),
        rotation: None,
        expiration: None,
        labels: HashMap::from([("".to_string(), "".to_string())]),
        topics: Vec::from([]),
        version_aliases: HashMap::from([("".to_string(), 0)]),
        annotations: HashMap::from([("".to_string(), "".to_string())]),
    });

    let response = client
        .get()
        .create_secret(tonic::Request::new(CreateSecretRequest {
            parent: format!("projects/{}", google_project_id),
            secret_id: format!("{}", secret_insertion),
            secret: secrets,
        }))
        .await?;
    println!("Response: {:#?}", response);

    Ok(())
}

pub async fn version(
    client: GoogleApi<SecretManagerServiceClient<GoogleAuthMiddleware>>,
    secret_name: String,
    secret_value: String
) -> Result<(), Box<dyn std::error::Error>> {
    let google_project_id: String = GoogleEnvironment::detect_google_project_id().await.expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");
    let pp = SecretPayload {
        data: secret_value.into(),
        data_crc32c: None,
    };

    let version = client
        .clone()
        .get()
        .add_secret_version(tonic::Request::new(AddSecretVersionRequest {
            parent: format!("projects/{}/secrets/{}", google_project_id, secret_name),
            payload: Some(pp),
        }))
        .await?;
    println!("Response: {:#?}", version);

    Ok(())
}
