extern crate google_gmail1 as gmail1;

use gmail1::{Gmail, oauth2, hyper, hyper_rustls, chrono, FieldMask};
use gmail1::api::Scope;
use gmail1::client::GetToken;

struct GmailProvider;

impl GmailProvider {
    fn new() -> Self {
        GmailProvider
    }

    async fn authenticate(&self) {
        println!("Authenticating with Gmail");
        let mut secret: oauth2::ApplicationSecret = oauth2::read_application_secret("data/client_secret.json")
            .await
            .expect("failed to read client secret from file");
        secret.redirect_uris = vec!["urn:ietf:wg:oauth:2.0:oob".to_string()];
        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::Interactive,
        ).build().await;
        match auth {
            Ok(auth) => {
                let token = auth.get_token(&[Scope::Gmai.as_ref()]).await
                    .expect("failed obtaining id token");
                println!("{:?}", token);
                println!("Authenticated with Gmail")
            },
            Err(e) => println!("Error authenticating with Gmail: {}", e),
        }
    }

    // fn build_hub(&self) {
    //     let mut hub = Gmail::new(hyper::Client::builder().build(
    //         hyper_rustls::HttpsConnectorBuilder::new()
    //             .with_native_roots()
    //             .https_or_http()
    //             .enable_http1()
    //             .enable_http2()
    //             .build()
    //     ), auth);
    // }

    async fn get_emails(&self) {
        println!("Getting emails from Gmail");
    }
}
#[tokio::main]
async fn main() {
    println!("OOD Emailer");
    GmailProvider::new().authenticate().await;
}
