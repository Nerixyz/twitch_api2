//! Gets all global emotes. Global emotes are Twitch-specific emoticons that every user can use in Twitch chat.
//! [`get-global-emotes`](https://dev.twitch.tv/docs/api/reference#get-global-emotes)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetGlobalEmotesRequest]
//!
//! To use this endpoint, construct a [`GetGlobalEmotesRequest`] with the [`GetGlobalEmotesRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::chat::get_global_emotes;
//! let request = get_global_emotes::GetGlobalEmotesRequest::default();
//! ```
//!
//! ## Response: [GlobalEmote]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, chat::get_global_emotes};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_global_emotes::GetGlobalEmotesRequest::default();
//! let response: Vec<helix::chat::GlobalEmote> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetGlobalEmotesRequest::parse_response(None, &request.get_uri(), response)`](GetGlobalEmotesRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel Emotes](super::get_global_emotes)
///
/// [`get-global-emotes`](https://dev.twitch.tv/docs/api/reference#get-global-emotes)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct GetGlobalEmotesRequest {}

/// Return Values for [Get Channel Emotes](super::get_global_emotes)
///
/// [`get-global-emotes`](https://dev.twitch.tv/docs/api/reference#get-global-emotes)
pub type GetChannelEmotesResponse = GlobalEmote;

impl Request for GetGlobalEmotesRequest {
    type Response = Vec<GetChannelEmotesResponse>;

    const PATH: &'static str = "chat/emotes/global";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetGlobalEmotesRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetGlobalEmotesRequest::default();

    // From twitch docs
    // FIXME: Example has ... and is malformed, uses [] in images
    let data = br#"
    {
      "data": [
        {
          "id": "196892",
          "name": "TwitchUnity",
          "images":
            {
              "url_1x": "https://static-cdn.jtvnw.net/emoticons/v1/196892/1.0",
              "url_2x": "https://static-cdn.jtvnw.net/emoticons/v1/196892/2.0",
              "url_4x": "https://static-cdn.jtvnw.net/emoticons/v1/196892/3.0"
            }
        }
      ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/emotes/global?"
    );

    dbg!(GetGlobalEmotesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
