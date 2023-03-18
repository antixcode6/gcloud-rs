use gcloud_sdk::{
    google::cloud::secretmanager::v1::secret_manager_service_client::SecretManagerServiceClient,
    GoogleApi, GoogleAuthMiddleware,
};

#[derive(Debug, PartialEq)]
pub enum GcloudProfile {
    Test(String),
    Dev(String),
    Prod(String),
}

pub struct GcloudClient {
    pub profile: GcloudProfile,
    pub client: GoogleApi<SecretManagerServiceClient<GoogleAuthMiddleware>>,
}
