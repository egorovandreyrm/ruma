pub mod v3 {
    use serde::{Deserialize, Serialize};

    use ruma_common::{
        api::{auth_scheme::AccessToken, request, response},
        metadata, OwnedServerName,
    };
    use ruma_common::authentication::TokenType;
    // "https://matrix.rpipro.tech/livekit-jwt-service",

    metadata! {
        method: POST,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            1.0 => "/livekit-jwt-service/sfu/get",
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OpenIdToken {
        pub access_token: String,
        pub token_type: TokenType,
        pub matrix_server_name: OwnedServerName,
    }

    //noinspection RsDetachedFile
    #[request(error = crate::Error)]
    pub struct Request {
        pub room: String,
        pub openid_token: OpenIdToken,
        pub device_id: String,
    }

    #[response(error = crate::Error)]
    pub struct Response {
        pub url: String,
        pub jwt: String,
    }

    impl Request {
        pub fn new(
            room: String,
            openid_token: OpenIdToken,
            device_id: String) -> Self {
            Self {
                room,
                openid_token,
                device_id
            }
        }
    }

    impl Response {
        pub fn new(url: String, jwt: String) -> Self {
            Self { url, jwt }
        }
    }
}
