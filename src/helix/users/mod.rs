#![doc(alias = "user")]
#![allow(deprecated)]
//! Helix endpoints regarding users
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, users::GetUsersRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetUsersRequest::builder()
//!     .login(vec!["justinfan1337".into()])
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod block_user;
pub mod get_user_block_list;
pub mod get_users;
pub mod get_users_follows;
pub mod unblock_user;

#[doc(inline)]
pub use block_user::{BlockUser, BlockUserRequest};
#[doc(inline)]
pub use get_user_block_list::{GetUserBlockListRequest, UserBlock};
#[doc(inline)]
pub use get_users::{GetUsersRequest, User};
#[doc(inline)]
pub use get_users_follows::{FollowRelationship, GetUsersFollowsRequest, UsersFollows};
#[doc(inline)]
pub use unblock_user::{UnblockUser, UnblockUserRequest};
