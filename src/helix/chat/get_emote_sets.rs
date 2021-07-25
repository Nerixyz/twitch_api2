//! Gets all Twitch emotes for one or more specific emote sets.
//! [`get-emote-sets`](https://dev.twitch.tv/docs/api/reference#get-emote-sets)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetEmoteSetsRequest]
//!
//! To use this endpoint, construct a [`GetEmoteSetsRequest`] with the [`GetEmoteSetsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::chat::get_emote_sets;
//! let request = get_emote_sets::GetEmoteSetsRequest::builder()
//!     .emote_set_id(vec!["1234".into()])
//!     .build();
//! ```
//!
//! ## Response: [Emote]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, chat::get_emote_sets};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_emote_sets::GetEmoteSetsRequest::builder()
//!     .emote_set_id(vec!["1234".into()])
//!     .build();
//! let response: Vec<helix::chat::get_emote_sets::Emote> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetEmoteSetsRequest::parse_response(None, &request.get_uri(), response)`](GetEmoteSetsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel Emotes](super::get_emote_sets)
///
/// [`get-emote-sets`](https://dev.twitch.tv/docs/api/reference#get-emote-sets)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetEmoteSetsRequest {
    // FIXME: twitch doc specifies maximum as 25, but it actually is 10
    /// The broadcaster whose emotes are being requested. Minimum: 1. Maximum: 10
    #[builder(setter(into))]
    pub emote_set_id: Vec<types::EmoteSetId>,
}

/// Return Values for [Get Channel Emotes](super::get_emote_sets)
///
/// [`get-emote-sets`](https://dev.twitch.tv/docs/api/reference#get-emote-sets)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Emote {
    /// Emote ID.
    id: types::EmoteId,
    /// Name of the emote a viewer types into Twitch chat for the image to appear.
    name: String,
    /// Object of image URLs for the emote.
    images: types::Image,
    // FIXME: Enumify?
    /// The type of emote.
    emote_type: String,
    /// ID of the emote set the emote belongs to.
    emote_set_id: types::EmoteSetId,
    /// User ID of the broadcaster who owns the emote.
    owner_id: types::UserId,
}

impl Request for GetEmoteSetsRequest {
    type Response = Vec<Emote>;

    const PATH: &'static str = "chat/emotes/set";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetEmoteSetsRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetEmoteSetsRequest::builder()
        .emote_set_id(vec!["301590448".into()])
        .build();

    // From twitch docs
    // FIXME: Example has ... and is malformed, uses [] in images
    let data = br#"
    {
      "data": [
        {
          "id": "304456832",
          "name": "twitchdevPitchfork",
          "images":
            {
              "url_1x": "https://static-cdn.jtvnw.net/emoticons/v1/304456832/1.0",
              "url_2x": "https://static-cdn.jtvnw.net/emoticons/v1/304456832/2.0",
              "url_4x": "https://static-cdn.jtvnw.net/emoticons/v1/304456832/3.0"
            },
          "emote_type": "subscriptions",
          "emote_set_id": "301590448",
          "owner_id": "141981764"
        }
      ]
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/chat/emotes/set?emote_set_id=301590448"
    );

    dbg!(GetEmoteSetsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
