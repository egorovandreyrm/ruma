pub mod v3 {
    use ruma_common::{
        api::{request, response, Metadata},
        authentication::TokenType,
        metadata, OwnedServerName,
    };

    use serde::{Deserialize, Serialize};

    use http::{
        HeaderName,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            1.0 => "/mappins/api/v1/map/pins/with/trackers",
        }
    };

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
    pub struct MapPin {
        pub id: u32,
        pub room_id: String,
        pub name: String,
        pub longitude: f64,
        pub latitude: f64,
        pub timestamp: u32,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
    pub struct TrackerLocation {
        pub id: u32,
        pub timestamp: String,
        pub longitude: f64,
        pub latitude: f64,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub altitude: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub bearing: Option<f32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub speed: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub accuracy: Option<f32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub hdop: Option<f64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub batt: Option<f64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<u32>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
    pub struct Tracker {
        pub id: u32,
        pub name: String,
        pub icon: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_location: Option<TrackerLocation>,
    }

    const ACCESS_TOKEN_HEADER_NAME: HeaderName = HeaderName::from_static("access_token");
    const MATRIX_SERVER_NAME_HEADER_NAME: HeaderName = HeaderName::from_static("matrix_server_name");

    #[request(error = crate::Error)]
    pub struct Request {
        #[ruma_api(header = ACCESS_TOKEN_HEADER_NAME)]
        pub access_token: String,

        #[ruma_api(header = MATRIX_SERVER_NAME_HEADER_NAME)]
        pub matrix_server_name: String,

        #[ruma_api(query)]
        pub room_id: String,
    }

    #[response(error = crate::Error)]
    pub struct Response {
        pub pins: Vec<MapPin>,
        pub trackers: Vec<Tracker>,
    }

    impl Request {
        pub fn new(access_token: String, matrix_server_name: String, room_id: String) -> Self {
            Self { access_token, matrix_server_name, room_id }
        }
    }

    impl Response {
        pub fn new(pins: Vec<MapPin>, trackers: Vec<Tracker>) -> Self {
            Self { pins, trackers }
        }
    }
}
