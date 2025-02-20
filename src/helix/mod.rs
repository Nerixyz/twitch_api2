//! Helix endpoints or the [New Twitch API](https://dev.twitch.tv/docs/api)
//!
//!
//! Aside from using [`HelixClient`] as described on [the crate documentation](crate),
//! you can decide to use this library without any specific client implementation.
//!
//! ```rust
//! use twitch_api2::helix::{self, Request, RequestGet, users::{GetUsersRequest, User}};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//!
//! let request = GetUsersRequest::builder()
//!     .login(vec!["justintv123".into()])
//!     .build();
//!
//! // Send it however you want
//! // Create a [`http::Response<Vec<u8>>`] with RequestGet::create_request, which takes an access token and a client_id
//! let response = send_http_request(request.create_request("accesstoken", "client_id")?)?;
//!
//! // then parse the response
//! let uri = request.get_uri()?;
//! let user: helix::Response<_, Vec<User>> = GetUsersRequest::parse_response(Some(request), &uri,response)?;
//! println!("{:#?}", user);
//! # Ok(())
//! # }
//! # fn send_http_request(_: http::Request<Vec<u8>>) -> Result<http::Response<Vec<u8>>,&'static str> {
//! # Ok(http::Response::builder().body(r#"{"data":[{"id":"141981764","login":"twitchdev","display_name":"TwitchDev","type":"","broadcaster_type":"partner","description":"Supportingthird-partydevelopersbuildingTwitchintegrationsfromchatbotstogameintegrations.","profile_image_url":"https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png","offline_image_url":"https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png","view_count":5980557,"email":"not-real@email.com","created_at":"2016-12-14T20:32:28.894263Z"}]}"#.as_bytes().to_owned()).unwrap())
//! # }
//! ```

// fn send_http_request(_: http::Request<Vec<u8>>) -> Result<http::Response<Vec<u8>>, &'static str> {
//     Ok(http::Response::builder().body(r#"{"data":[{"id":"141981764","login":"twitchdev","display_name":"TwitchDev","type":"","broadcaster_type":"partner","description":"Supportingthird-partydevelopersbuildingTwitchintegrationsfromchatbotstogameintegrations.","profile_image_url":"https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png","offline_image_url":"https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png","view_count":5980557,"email":"not-real@email.com","created_at":"2016-12-14T20:32:28.894263Z"}]}"#.as_bytes().to_owned()).unwrap())
// }

use serde::Deserialize;
use std::{convert::TryInto, str::FromStr};
#[cfg(feature = "twitch_oauth2")]
use twitch_oauth2::TwitchToken;
#[cfg(all(feature = "client"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "helix"))))]
mod client_ext;

#[cfg(all(feature = "client"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "helix"))))]
pub use client_ext::make_stream;

pub mod bits;
pub mod channels;
pub mod chat;
pub mod clips;
#[cfg(feature = "eventsub")]
#[cfg_attr(nightly, doc(cfg(feature = "eventsub")))]
pub mod eventsub;
pub mod games;
pub mod goals;
pub mod hypetrain;
pub mod moderation;
pub mod points;
pub mod polls;
pub mod predictions;
pub mod schedule;
pub mod search;
pub mod streams;
pub mod subscriptions;
pub mod tags;
pub mod teams;
pub mod users;
pub mod videos;

pub(crate) mod ser;
pub(crate) use crate::deserialize_default_from_null;
use crate::{parse_json, parse_json_value};
pub use ser::Error as SerializeError;

#[doc(no_inline)]
#[cfg(feature = "twitch_oauth2")]
pub use twitch_oauth2::Scope;

/// Client for Helix or the [New Twitch API](https://dev.twitch.tv/docs/api)
///
/// Provides [`HelixClient::req_get`] for requesting endpoints which uses [GET method][RequestGet].
///
///
/// Most [clients][crate::HttpClient] will be able to use the `'static` lifetime
///
/// ```rust,no_run
/// # use twitch_api2::{HelixClient}; pub mod reqwest {pub type Client = twitch_api2::client::DummyHttpClient;}
/// pub struct MyStruct {
///     twitch: HelixClient<'static, reqwest::Client>,
///     token: twitch_oauth2::AppAccessToken,
/// }
/// // etc
/// ```
///
/// See [`HttpClient`][crate::HttpClient] for implemented http clients, you can also define your own if needed.
///
/// # Examples
///
/// Get a [user](users::User) from their login name.
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # pub mod reqwest {pub type Client = twitch_api2::client::DummyHttpClient;}
/// let client: HelixClient<'static, reqwest::Client> = HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
/// use twitch_api2::helix::{users::User, HelixClient};
/// let user: Option<User> = client
///     .get_user_from_login("justintv".to_string(), &token)
///     .await
///     .unwrap();
/// # Ok(()) }
/// ```
#[cfg(all(feature = "client"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "helix"))))]
#[derive(Clone)]
pub struct HelixClient<'a, C>
where C: crate::HttpClient<'a> {
    pub(crate) client: C,
    _pd: std::marker::PhantomData<&'a ()>, // TODO: Implement rate limiter...
}

#[derive(PartialEq, Deserialize, Debug)]
struct InnerResponse<D> {
    data: D,
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    #[serde(default)]
    pagination: Pagination,
    #[serde(default)]
    total: Option<i64>,
    #[serde(default, flatten)]
    other: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Deserialize, Debug)]
#[cfg(feature = "unsupported")]
#[cfg_attr(nightly, doc(cfg(feature = "unsupported")))]
struct CustomInnerResponse<'a> {
    #[serde(borrow)]
    data: &'a serde_json::value::RawValue,
    #[serde(default)]
    pagination: Pagination,
    #[serde(default)]
    total: Option<i64>,
    // FIXME: There is an issue with RawValue on flatten maps. https://github.com/serde-rs/json/issues/599
    #[serde(flatten, default)]
    other: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize, Clone, Debug)]
struct HelixRequestError {
    error: String,
    status: u16,
    message: String,
}

#[cfg(feature = "client")]
impl<'a, C: crate::HttpClient<'a>> HelixClient<'a, C> {
    /// Create a new client with an existing client
    pub fn with_client(client: C) -> HelixClient<'a, C> {
        HelixClient {
            client,
            _pd: std::marker::PhantomData::default(),
        }
    }

    /// Create a new [`HelixClient`] with a default [`HttpClient`][crate::HttpClient]
    pub fn new() -> HelixClient<'a, C>
    where C: crate::client::ClientDefault<'a> {
        let client = C::default_client();
        HelixClient::with_client(client)
    }

    /// Retrieve a clone of the [`HttpClient`][crate::HttpClient] inside this [`HelixClient`]
    pub fn clone_client(&self) -> C
    where C: Clone {
        self.client.clone()
    }

    /// Retrieve a reference of the [`HttpClient`][crate::HttpClient] inside this [`HelixClient`]
    pub fn get_client(&self) -> &C { &self.client }

    /// Request on a valid [`RequestGet`] endpoint
    ///
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// #   use twitch_api2::helix::{HelixClient, channels};
    /// #   let token = Box::new(twitch_oauth2::UserToken::from_existing_unchecked(
    /// #       twitch_oauth2::AccessToken::new("totallyvalidtoken".to_string()), None,
    /// #       twitch_oauth2::ClientId::new("validclientid".to_string()), None, "justintv".to_string(), "1337".to_string(), None, None));
    ///     let req = channels::GetChannelInformationRequest::builder().broadcaster_id("123456").build();
    ///     let client = HelixClient::new();
    /// # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
    ///
    ///     let response = client.req_get(req, &token).await;
    /// # }
    /// # // fn main() {run()}
    /// ```
    pub async fn req_get<R, D, T>(
        &'a self,
        request: R,
        token: &T,
    ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestGet,
        D: serde::de::DeserializeOwned + PartialEq,
        T: TwitchToken + ?Sized,
        C: Send,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    }

    /// Request on a valid [`RequestPost`] endpoint
    pub async fn req_post<R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestPost<Body = B>,
        B: HelixRequestBody,
        D: serde::de::DeserializeOwned + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    }

    /// Request on a valid [`RequestPatch`] endpoint
    pub async fn req_patch<R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestPatch<Body = B>,
        B: HelixRequestBody,
        D: serde::de::DeserializeOwned + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    }

    /// Request on a valid [`RequestDelete`] endpoint
    pub async fn req_delete<R, D, T>(
        &'a self,
        request: R,
        token: &T,
    ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestDelete,
        D: serde::de::DeserializeOwned + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    }

    /// Request on a valid [`RequestPut`] endpoint
    pub async fn req_put<R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<Response<R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request<Response = D> + Request + RequestPut<Body = B>,
        B: HelixRequestBody,
        D: serde::de::DeserializeOwned + PartialEq,
        T: TwitchToken + ?Sized,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        <R>::parse_response(Some(request), &uri, response).map_err(Into::into)
    }
}

#[cfg(all(feature = "client", feature = "unsupported"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "unsupported"))))]
impl<'a, C: crate::HttpClient<'a>> HelixClient<'a, C> {
    /// Request on a valid [`RequestGet`] endpoint, with the ability to return borrowed data and specific fields.
    pub async fn req_get_custom<'d, R, D, T>(
        &'a self,
        request: R,
        token: &T,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestGet,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        {
            let request = Some(request);
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestGetError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            //eprintln!("\n\nmessage is ------------ {} ------------", text);
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestGetError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                }
                .into());
            }
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestGetError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request,
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }

    /// Request on a valid [`RequestPost`] endpoint, with the ability to return borrowed data and specific fields.
    pub async fn req_post_custom<'d, R, B, D, T>(
        &'a self,
        request: R,
        body: B,
        token: &T,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestPost + RequestPost<Body = B>,
        B: HelixRequestBody,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        {
            let request = Some(request);
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestPostError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            //eprintln!("\n\nmessage is ------------ {} ------------", text);
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestPostError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                }
                .into());
            }
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestPostError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request,
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }

    /// Request on a valid [`RequestPatch`] endpoint, with the ability to return borrowed data and specific fields.
    ///
    /// # Notes
    ///
    /// This is probably not useful, as `PATCH` endpoints do not usually return json
    pub async fn req_patch_custom<'d, R, B, D, T, F>(
        &'a self,
        request: R,
        body: B,
        token: &T,
        function: F,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestPatch + RequestPatch<Body = B>,
        B: HelixRequestBody,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
        F: Fn(&R, &http::Uri, &str, http::StatusCode) -> Result<(), HelixRequestPatchError>,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        {
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestPatchError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestPatchError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                }
                .into());
            }
            function(&request, uri, text, response.status())?;
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestPatchError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request: Some(request),
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }

    /// Request on a valid [`RequestDelete`] endpoint, with the ability to return borrowed data and specific fields.
    ///
    /// # Notes
    ///
    /// This is probably not useful, as `DELETE` endpoints do not usually return json
    pub async fn req_delete_custom<'d, R, D, T, F>(
        &'a self,
        request: R,
        token: &T,
        function: F,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestDelete,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
        F: Fn(&R, &http::Uri, &str, http::StatusCode) -> Result<(), HelixRequestDeleteError>,
    {
        let req = request.create_request(token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        {
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestDeleteError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestDeleteError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                }
                .into());
            }
            function(&request, uri, text, response.status())?;
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestPatchError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request: Some(request),
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }

    /// Request on a valid [`RequestPut`] endpoint, with the ability to return borrowed data and specific fields.
    ///
    /// # Notes
    ///
    /// This is probably not useful, as `PUT` endpoints do not usually return json
    pub async fn req_put_custom<'d, R, B, D, T, F>(
        &'a self,
        request: R,
        body: B,
        token: &T,
        function: F,
    ) -> Result<CustomResponse<'d, R, D>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    where
        R: Request + RequestPut + RequestPut<Body = B>,
        B: HelixRequestBody,
        D: serde::de::Deserialize<'d> + 'd,
        T: TwitchToken + ?Sized,
        C: Send,
        F: Fn(&R, &http::Uri, &str, http::StatusCode) -> Result<(), HelixRequestDeleteError>,
    {
        let req =
            request.create_request(body, token.token().secret(), token.client_id().as_str())?;
        let uri = req.uri().clone();
        let response = self
            .client
            .req(req)
            .await
            .map_err(ClientRequestError::RequestError)?;
        {
            let uri = &uri;
            let text = std::str::from_utf8(response.body()).map_err(|e| {
                HelixRequestPutError::Utf8Error(response.body().clone(), e, uri.clone())
            })?;
            if let Ok(HelixRequestError {
                error,
                status,
                message,
            }) = parse_json::<HelixRequestError>(text, false)
            {
                return Err(HelixRequestPutError::Error {
                    error,
                    status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                    message,
                    uri: uri.clone(),
                    body: response.body().clone(),
                }
                .into());
            }
            function(&request, uri, text, response.status())?;
            let response: CustomInnerResponse<'_> = crate::parse_json(text, true).map_err(|e| {
                HelixRequestPatchError::DeserializeError(
                    text.to_owned(),
                    e,
                    uri.clone(),
                    response.status(),
                )
            })?;
            Ok(CustomResponse {
                pagination: response.pagination.cursor,
                request: Some(request),
                total: response.total,
                other: response.other,
                raw_data: response.data.to_owned(),
                pd: <_>::default(),
            })
        }
    }
}

#[cfg(feature = "client")]
impl<C: crate::HttpClient<'static> + crate::client::ClientDefault<'static>> Default
    for HelixClient<'static, C>
{
    fn default() -> Self { Self::new() }
}

/// Deserialize "" as <T as Default>::Default
fn deserialize_none_from_empty_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: serde::de::DeserializeOwned, {
    let val = serde_json::Value::deserialize(deserializer)?;
    match val {
        serde_json::Value::String(string) if string.is_empty() => Ok(None),
        other => Ok(parse_json_value(other, true).map_err(serde::de::Error::custom)?),
    }
}

/// A request is a Twitch endpoint, see [New Twitch API](https://dev.twitch.tv/docs/api/reference) reference
#[async_trait::async_trait]
pub trait Request: serde::Serialize {
    /// The path to the endpoint relative to the helix root. eg. `channels` for [Get Channel Information](https://dev.twitch.tv/docs/api/reference#get-channel-information)
    const PATH: &'static str;
    /// Scopes needed by this endpoint
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope];
    /// Optional scopes needed by this endpoint
    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    /// Response type. twitch's response will  deserialize to this.
    type Response: serde::de::DeserializeOwned + PartialEq;
    /// Defines layout of the url parameters.
    fn query(&self) -> Result<String, ser::Error> { ser::to_string(&self) }
    /// Returns full URI for the request, including query parameters.
    fn get_uri(&self) -> Result<http::Uri, InvalidUri> {
        let query = self.query()?;
        let url = crate::TWITCH_HELIX_URL
            .join(<Self as Request>::PATH)
            .map(|mut u| {
                u.set_query(Some(&query));
                u
            })?;
        http::Uri::from_str(url.as_str()).map_err(Into::into)
    }
    /// Returns bare URI for the request, NOT including query parameters.
    fn get_bare_uri() -> Result<http::Uri, InvalidUri> {
        let url = crate::TWITCH_HELIX_URL.join(<Self as Request>::PATH)?;
        http::Uri::from_str(url.as_str()).map_err(Into::into)
    }
}

/// Helix endpoint POSTs information
pub trait RequestPost: Request {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        //eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
                CreateRequestError::Custom("Could not make token into headervalue".into())
            })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::POST)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(body)
            .map_err(Into::into)
    }

    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response(
        // FIXME: Is this really needed? Its currently only used for error reporting.
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPostError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(response.body()).map_err(|e| {
            HelixRequestPostError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestPostError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        <Self as RequestPost>::parse_inner_response(request, uri, text, response.status())
    }

    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPostError>
    where
        Self: Sized,
    {
        let response: InnerResponse<<Self as Request>::Response> = parse_json(response, true)
            .map_err(|e| {
                HelixRequestPostError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(Response {
            data: response.data,
            pagination: response.pagination.cursor,
            request,
            total: response.total,
            other: None,
        })
    }
}

/// Helix endpoint PATCHs information
pub trait RequestPatch: Request {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        // eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
                CreateRequestError::Custom("Could not make token into headervalue".into())
            })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::PATCH)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(body)
            .map_err(Into::into)
    }

    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response(
        // FIXME: Is this really needed? Its currently only used for error reporting.
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPatchError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(response.body()).map_err(|e| {
            HelixRequestPatchError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestPatchError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        <Self as RequestPatch>::parse_inner_response(request, uri, text, response.status())
    }

    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPatchError>
    where
        Self: Sized;
}

/// Helix endpoint DELETEs information
pub trait RequestDelete: Request {
    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
                CreateRequestError::Custom("Could not make token into headervalue".into())
            })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::DELETE)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(Vec::with_capacity(0))
            .map_err(Into::into)
    }
    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response(
        // FIXME: Is this really needed? Its currently only used for error reporting.
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestDeleteError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(response.body()).map_err(|e| {
            HelixRequestDeleteError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestDeleteError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        <Self as RequestDelete>::parse_inner_response(request, uri, text, response.status())
    }
    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestDeleteError>
    where
        Self: Sized;
}

/// Helix endpoint PUTs information
pub trait RequestPut: Request {
    /// Body parameters
    type Body: HelixRequestBody;

    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        body: Self::Body,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let body = body.try_to_body()?;
        // eprintln!("\n\nbody is ------------ {} ------------", body);

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
                CreateRequestError::Custom("Could not make token into headervalue".into())
            })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::PUT)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(body)
            .map_err(Into::into)
    }

    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response(
        // FIXME: Is this really needed? Its currently only used for error reporting.
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPutError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(response.body()).map_err(|e| {
            HelixRequestPutError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestPutError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
                body: response.body().clone(),
            });
        }
        <Self as RequestPut>::parse_inner_response(request, uri, text, response.status())
    }

    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestPutError>
    where
        Self: Sized;
}

/// Helix endpoint GETs information
pub trait RequestGet: Request {
    /// Create a [`http::Request`] from this [`Request`] in your client
    fn create_request(
        &self,
        token: &str,
        client_id: &str,
    ) -> Result<http::Request<Vec<u8>>, CreateRequestError> {
        let uri = self.get_uri()?;

        let mut bearer =
            http::HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|_| {
                CreateRequestError::Custom("Could not make token into headervalue".into())
            })?;
        bearer.set_sensitive(true);
        http::Request::builder()
            .method(http::Method::GET)
            .uri(uri)
            .header("Client-ID", client_id)
            .header("Content-Type", "application/json")
            .header(http::header::AUTHORIZATION, bearer)
            .body(Vec::with_capacity(0))
            .map_err(Into::into)
    }

    /// Parse response.
    ///
    /// # Notes
    ///
    /// Pass in the request to enable [pagination](Response::get_next) if supported.
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestGetError>
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(response.body()).map_err(|e| {
            HelixRequestGetError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        //eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(HelixRequestError {
            error,
            status,
            message,
        }) = parse_json::<HelixRequestError>(text, false)
        {
            return Err(HelixRequestGetError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }
        <Self as RequestGet>::parse_inner_response(request, uri, text, response.status())
    }

    /// Parse a response string into the response.
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<Response<Self, <Self as Request>::Response>, HelixRequestGetError>
    where
        Self: Sized,
    {
        let response: InnerResponse<_> = parse_json(response, true).map_err(|e| {
            HelixRequestGetError::DeserializeError(response.to_string(), e, uri.clone(), status)
        })?;
        Ok(Response {
            data: response.data,
            pagination: response.pagination.cursor,
            request,
            total: response.total,
            other: response.other,
        })
    }
}

/// Response retrieved from endpoint. Data is the type in [`Request::Response`]
#[derive(PartialEq, Debug)]
#[non_exhaustive]
pub struct Response<R, D>
where
    R: Request,
    D: serde::de::DeserializeOwned + PartialEq, {
    /// Twitch's response field for `data`.
    pub data: D,
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    pub pagination: Option<Cursor>,
    /// The request that was sent, used for [pagination](Paginated).
    pub request: Option<R>,
    /// Response would return this many results if fully paginated. Sometimes this is not emmitted or correct for this purpose, in those cases, this value will be `None`.
    pub total: Option<i64>,
    /// Fields which are not part of the data response, but are returned by the endpoint.
    ///
    /// See for example [Get Broadcaster Subscriptions](https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions) which returns this.
    pub other: Option<serde_json::Map<String, serde_json::Value>>,
}

impl<R, D> Response<R, D>
where
    R: Request,
    D: serde::de::DeserializeOwned + PartialEq,
{
    /// Get a field from the response that is not part of `data`.
    pub fn get_other<Q, V>(&self, key: &Q) -> Result<Option<V>, serde_json::Error>
    where
        String: std::borrow::Borrow<Q>,
        Q: ?Sized + Ord + Eq + std::hash::Hash,
        V: serde::de::DeserializeOwned, {
        use std::borrow::Borrow as _;
        match &key {
            total if &String::from("total").borrow() == total => {
                if let Some(total) = self.total {
                    let total = serde_json::json!(total);
                    Some(serde_json::from_value(total)).transpose()
                } else {
                    Ok(None)
                }
            }
            _ => self
                .other
                .as_ref()
                .and_then(|map| map.get(key.borrow()))
                .map(|v| serde_json::from_value(v.clone()))
                .transpose(),
        }
    }
}

/// Custom response retrieved from endpoint, used for specializing responses
#[cfg(all(feature = "client", feature = "unsupported"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "unsupported"))))]
#[non_exhaustive]
pub struct CustomResponse<'d, R, D>
where
    R: Request,
    D: 'd, {
    /// A cursor value, to be used in a subsequent request to specify the starting point of the next set of results.
    pub pagination: Option<Cursor>,
    /// The request that was sent, used for [pagination](Paginated).
    pub request: Option<R>,
    /// Response would return this many results if fully paginated. Sometimes this is not emmitted or correct for this purpose, in those cases, this value will be `None`.
    pub total: Option<i64>,
    /// Other fields that are part of the response, but unknown.
    ///
    /// Unfortunately, this [can't be borrowed](https://github.com/serde-rs/json/issues/599).
    pub other: serde_json::Map<String, serde_json::Value>,
    /// The owned data. Use [`CustomResponse::data()`] to deserialize.
    pub raw_data: Box<serde_json::value::RawValue>,
    pd: std::marker::PhantomData<&'d D>,
}

#[cfg(all(feature = "client", feature = "unsupported"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "unsupported"))))]
impl<'d, R, D> CustomResponse<'d, R, D>
where
    R: Request,
    D: 'd + serde::Deserialize<'d>,
{
    /// Deserialize the data
    pub fn data(&'d self) -> Result<D, serde_json::Error> {
        serde_json::from_str(self.raw_data.get())
    }
}

impl<R, D, T> Response<R, D>
where
    R: Request,
    D: IntoIterator<Item = T> + PartialEq + serde::de::DeserializeOwned,
{
    /// Get first result of this response.
    pub fn first(self) -> Option<T> { self.data.into_iter().next() }
}

// impl<R, D, T> CustomResponse<'_, R, D>
// where
//     R: Request,
//     D: IntoIterator<Item = T>,
// {
//     /// Get first result of this response.
//     pub fn first(self) -> Option<T> { self.data().into_iter().next() }
// }

#[cfg(feature = "client")]
impl<R, D> Response<R, D>
where
    R: Request<Response = D> + Clone + Paginated + RequestGet + std::fmt::Debug,
    D: serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    /// Get the next page in the responses.
    pub async fn get_next<'a, C: crate::HttpClient<'a>>(
        self,
        client: &'a HelixClient<'a, C>,
        token: &(impl TwitchToken + ?Sized),
    ) -> Result<Option<Response<R, D>>, ClientRequestError<<C as crate::HttpClient<'a>>::Error>>
    {
        if let Some(mut req) = self.request.clone() {
            if self.pagination.is_some() {
                req.set_pagination(self.pagination);
                let res = client.req_get(req, token).await.map(Some);
                if let Ok(Some(r)) = res {
                    // FIXME: Workaround for https://github.com/twitchdev/issues/issues/18
                    if r.data == self.data {
                        Ok(None)
                    } else {
                        Ok(Some(r))
                    }
                } else {
                    res
                }
            } else {
                Ok(None)
            }
        } else {
            // TODO: Make into proper error
            Err(ClientRequestError::Custom(
                "no source request attached".into(),
            ))
        }
    }
}

/// A request that can be paginated.
pub trait Paginated: Request {
    /// Should returns the current pagination cursor.
    ///
    /// # Notes
    ///
    /// Pass [`Option::None`] if no cursor is found.
    fn set_pagination(&mut self, cursor: Option<Cursor>);
}

/// A cursor for pagination. This is needed because of how pagination is represented in the [New Twitch API](https://dev.twitch.tv/docs/api)
#[derive(PartialEq, Deserialize, Debug, Clone, Default)]
struct Pagination {
    #[serde(default)]
    cursor: Option<Cursor>,
}

/// A cursor is a pointer to the current "page" in the twitch api pagination
#[aliri_braid::braid(serde)]
pub struct Cursor;

/// Errors for [`HelixClient::req_get`] and similar functions.
#[derive(thiserror::Error, Debug)]
// #[derive(displaydoc::Display)] https://github.com/yaahc/displaydoc/issues/15
pub enum ClientRequestError<RE: std::error::Error + Send + Sync + 'static> {
    /// Request failed from reqwests side
    #[error("request failed from reqwests side")]
    RequestError(RE),
    /// No pagination found
    #[error("no pagination found")]
    NoPage,
    /// Could not create request
    #[error("could not create request")]
    CreateRequestError(#[from] CreateRequestError),
    /// Got error from GET response
    #[error(transparent)]
    HelixRequestGetError(#[from] HelixRequestGetError),
    /// Got error from PUT response
    #[error(transparent)]
    HelixRequestPutError(#[from] HelixRequestPutError),
    /// Got error from POST response
    #[error(transparent)]
    HelixRequestPostError(#[from] HelixRequestPostError),
    /// Got error from PATCH response
    #[error(transparent)]
    HelixRequestPatchError(#[from] HelixRequestPatchError),
    /// Got error from DELETE response
    #[error(transparent)]
    HelixRequestDeleteError(#[from] HelixRequestDeleteError),
    /// Custom error
    #[error("{0}")]
    Custom(std::borrow::Cow<'static, str>),
}
/// Could not create request
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum CreateRequestError {
    /// http crate returned an error
    HttpError(#[from] http::Error),
    /// serialization of body failed
    SerializeError(#[from] BodyError),
    /// could not assemble URI for request
    InvalidUri(#[from] InvalidUri),
    /// {0}
    Custom(std::borrow::Cow<'static, str>),
}

/// Errors that can happen when creating [`http::Uri`] for [`Request`]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum InvalidUri {
    /// URI could not be parsed
    UriParseError(#[from] http::uri::InvalidUri),
    /// could not assemble URI for request
    UrlError(#[from] url::ParseError),
    /// could not serialize request to query
    QuerySerializeError(#[from] ser::Error),
}

/// Could not parse GET response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum HelixRequestGetError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `GET {uri}`
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
    },
    /// could not parse response as utf8 when calling `GET {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `GET {2}` with response: {3} - {0:?}
    DeserializeError(
        String,
        #[source] crate::DeserError,
        http::Uri,
        http::StatusCode,
    ),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse PUT response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum HelixRequestPutError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `PUT {uri}` with a body
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent to PUT response
        body: Vec<u8>,
    },
    /// could not parse response as utf8 when calling `PUT {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `PUT {2}` with response: {3} - {0:?}
    DeserializeError(
        String,
        #[source] crate::DeserError,
        http::Uri,
        http::StatusCode,
    ),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse POST response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum HelixRequestPostError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `POST {uri}` with a body
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent to POST response
        body: Vec<u8>,
    },
    /// could not parse response as utf8 when calling `POST {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `POST {2}` with response: {3} - {0:?}
    DeserializeError(
        String,
        #[source] crate::DeserError,
        http::Uri,
        http::StatusCode,
    ),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse PATCH response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum HelixRequestPatchError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `PATCH {uri}` with a body
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent to POST response
        body: Vec<u8>,
    },
    /// could not parse response as utf8 when calling `POST {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// deserialization failed when processing request response calling `POST {2}` with response: {3} - {0:?}
    DeserializeError(
        String,
        #[source] crate::DeserError,
        http::Uri,
        http::StatusCode,
    ),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Could not parse DELETE response
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum HelixRequestDeleteError {
    /// helix returned error {status:?} - {error}: {message:?} when calling `DELETE {uri}`
    Error {
        /// Error message related to status code
        error: String,
        /// Status code of error, usually 400-499
        status: http::StatusCode,
        /// Error message from Twitch
        message: String,
        /// URI to the endpoint
        uri: http::Uri,
        /// Body sent to DELETE response
        body: Vec<u8>,
    },
    /// could not parse response as utf8 when calling `DELETE {2}`
    Utf8Error(Vec<u8>, #[source] std::str::Utf8Error, http::Uri),
    /// invalid or unexpected response from twitch.
    InvalidResponse {
        /// Reason for error
        reason: &'static str,
        /// Response text
        response: String,
        /// Status Code
        status: http::StatusCode,
        /// Uri to endpoint
        uri: http::Uri,
    },
}

/// Errors that can happen when creating a body
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum BodyError {
    /// could not serialize as json
    JsonError(#[from] serde_json::Error),
    /// could not serialize to query
    QuerySerializeError(#[from] ser::Error),
    /// uri is invalid
    InvalidUri(#[from] InvalidUri),
}

/// Create a body. Used for specializing request bodies
pub trait HelixRequestBody {
    /// Create the body
    fn try_to_body(&self) -> Result<Vec<u8>, BodyError>;
}

/// An empty body.
///
/// Implements [`HelixRequestBody::try_to_body`], returning an empty vector
#[derive(Default, Clone, Copy)]
pub struct EmptyBody;

impl HelixRequestBody for EmptyBody {
    fn try_to_body(&self) -> Result<Vec<u8>, BodyError> { Ok(vec![]) }
}

// TODO: I would want specialization for this. For now, to override this behavior for a body, we specify a sealed trait
impl<T> HelixRequestBody for T
where T: serde::Serialize + private::SealedSerialize
{
    fn try_to_body(&self) -> Result<Vec<u8>, BodyError> {
        serde_json::to_vec(&self).map_err(Into::into)
    }
}

pub(crate) mod private {
    pub trait SealedSerialize {}
}
