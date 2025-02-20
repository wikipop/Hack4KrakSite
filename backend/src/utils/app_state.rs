use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId};
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub database: DatabaseConnection,
    pub github_oauth_client: BasicClient,
}

impl AppState {
    pub fn with_database(database: DatabaseConnection) -> AppState {
        AppState {
            database,
            github_oauth_client: BasicClient::new(
                ClientId::new("test".to_string()),
                None,
                AuthUrl::new("http://auth".to_string()).unwrap(),
                None,
            ),
        }
    }
}
