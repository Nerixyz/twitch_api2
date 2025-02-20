//! EventSub events and their types

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::*;

macro_rules! is_thing {
    ($s:expr, $thing:ident) => {
        is_thing!(@inner $s, $thing;
            channel::ChannelUpdateV1;
            channel::ChannelFollowV1;
            channel::ChannelSubscribeV1;
            channel::ChannelCheerV1;
            channel::ChannelBanV1;
            channel::ChannelUnbanV1;
            channel::ChannelPointsCustomRewardAddV1;
            channel::ChannelPointsCustomRewardUpdateV1;
            channel::ChannelPointsCustomRewardRemoveV1;
            channel::ChannelPointsCustomRewardRedemptionAddV1;
            channel::ChannelPointsCustomRewardRedemptionUpdateV1;
            channel::ChannelPollBeginV1;
            channel::ChannelPollProgressV1;
            channel::ChannelPollEndV1;
            channel::ChannelPredictionBeginV1;
            channel::ChannelPredictionProgressV1;
            channel::ChannelPredictionLockV1;
            channel::ChannelPredictionEndV1;
            channel::ChannelRaidV1;
            channel::ChannelSubscriptionEndV1;
            channel::ChannelSubscriptionGiftV1;
            channel::ChannelSubscriptionMessageV1;
            channel::ChannelGoalBeginV1;
            channel::ChannelGoalProgressV1;
            channel::ChannelGoalEndV1;
            channel::ChannelHypeTrainBeginV1;
            channel::ChannelHypeTrainProgressV1;
            channel::ChannelHypeTrainEndV1;
            stream::StreamOnlineV1;
            stream::StreamOfflineV1;
            user::UserUpdateV1;
            user::UserAuthorizationGrantV1;
            user::UserAuthorizationRevokeV1;
        )
    };
    (@inner $s:expr, $thing:ident; $($module:ident::$event:ident);* $(;)?) => {
        match $s {
            $(Event::$event(Payload { message : Message::$thing(..), ..}) => true,)*
            _ => false,
        }
    };
}

/// Event types
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum EventType {
    /// `channel.update` subscription type sends notifications when a broadcaster updates the category, title, mature flag, or broadcast language for their channel.
    #[serde(rename = "channel.update")]
    ChannelUpdate,
    /// `channel.follow`: a specified channel receives a follow.
    #[serde(rename = "channel.follow")]
    ChannelFollow,
    /// `channel.subscribe`: a specified channel receives a subscriber. This does not include resubscribes.
    #[serde(rename = "channel.subscribe")]
    ChannelSubscribe,
    /// `channel.cheer`: a user cheers on the specified channel.
    #[serde(rename = "channel.cheer")]
    ChannelCheer,
    /// `channel.ban`: a viewer is banned from the specified channel.
    #[serde(rename = "channel.ban")]
    ChannelBan,
    /// `channel.unban`: a viewer is unbanned from the specified channel.
    #[serde(rename = "channel.unban")]
    ChannelUnban,
    /// `channel.channel_points_custom_reward.add`: a custom channel points reward has been created for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.add")]
    ChannelPointsCustomRewardAdd,
    /// `channel.channel_points_custom_reward.update`: a custom channel points reward has been updated for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.update")]
    ChannelPointsCustomRewardUpdate,
    /// `channel.channel_points_custom_reward.remove`: a custom channel points reward has been removed from the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward.remove")]
    ChannelPointsCustomRewardRemove,
    /// `channel.channel_points_custom_reward_redemption.add`: a viewer has redeemed a custom channel points reward on the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward_redemption.add")]
    ChannelPointsCustomRewardRedemptionAdd,
    /// `channel.channel_points_custom_reward_redemption.update`: a redemption of a channel points custom reward has been updated for the specified channel.
    #[serde(rename = "channel.channel_points_custom_reward_redemption.update")]
    ChannelPointsCustomRewardRedemptionUpdate,
    /// `channel.poll.begin`: a poll begins on the specified channel.
    #[serde(rename = "channel.poll.begin")]
    ChannelPollBegin,
    /// `channel.poll.progress`: a user responds to a poll on the specified channel.
    #[serde(rename = "channel.poll.progress")]
    ChannelPollProgress,
    /// `channel.poll.end`: a poll ends on the specified channel.
    #[serde(rename = "channel.poll.end")]
    ChannelPollEnd,
    /// `channel.prediction.begin`: a Prediction begins on the specified channel
    #[serde(rename = "channel.prediction.begin")]
    ChannelPredictionBegin,
    /// `channel.prediction.progress`: a user participates in a Prediction on the specified channel.
    #[serde(rename = "channel.prediction.progress")]
    ChannelPredictionProgress,
    /// `channel.prediction.lock`: a Prediction is locked on the specified channel.
    #[serde(rename = "channel.prediction.lock")]
    ChannelPredictionLock,
    /// `channel.prediction.end`: a Prediction ends on the specified channel.
    #[serde(rename = "channel.prediction.end")]
    ChannelPredictionEnd,
    /// `channel.raid`: a broadcaster raids another broadcaster’s channel.
    #[serde(rename = "channel.raid")]
    ChannelRaid,
    /// `channel.subscription.end`: a subscription to the specified channel expires.
    #[serde(rename = "channel.subscription.end")]
    ChannelSubscriptionEnd,
    /// `channel.subscription.gift`: a user gives one or more gifted subscriptions in a channel.
    #[serde(rename = "channel.subscription.gift")]
    ChannelSubscriptionGift,
    /// `channel.subscription.gift`: a user sends a resubscription chat message in a specific channel
    #[serde(rename = "channel.subscription.message")]
    ChannelSubscriptionMessage,
    /// `channel.goal.begin`: a goal begins on the specified channel.
    #[serde(rename = "channel.goal.begin")]
    ChannelGoalBegin,
    /// `channel.goal.progress`: a goal makes progress on the specified channel.
    #[serde(rename = "channel.goal.progress")]
    ChannelGoalProgress,
    /// `channel.goal.end`: a goal ends on the specified channel.
    #[serde(rename = "channel.goal.end")]
    ChannelGoalEnd,
    /// `channel.hype_train.begin`: a hype train begins on the specified channel.
    #[serde(rename = "channel.hype_train.begin")]
    ChannelHypeTrainBegin,
    /// `channel.hype_train.progress`: a hype train makes progress on the specified channel.
    #[serde(rename = "channel.hype_train.progress")]
    ChannelHypeTrainProgress,
    /// `channel.hype_train.end`: a hype train ends on the specified channel.
    #[serde(rename = "channel.hype_train.end")]
    ChannelHypeTrainEnd,
    /// `stream.online`: the specified broadcaster starts a stream.
    #[serde(rename = "stream.online")]
    StreamOnline,
    /// `stream.online`: the specified broadcaster stops a stream.
    #[serde(rename = "stream.offline")]
    StreamOffline,
    /// `user.update`: user updates their account.
    #[serde(rename = "user.update")]
    UserUpdate,
    /// `user.authorization.revoke`: a user has revoked authorization for your client id. Use this webhook to meet government requirements for handling user data, such as GDPR, LGPD, or CCPA.
    #[serde(rename = "user.authorization.revoke")]
    UserAuthorizationRevoke,
    /// `user.authorization.revoke`: a user’s authorization has been granted to your client id.
    #[serde(rename = "user.authorization.grant")]
    UserAuthorizationGrant,
}

/// A notification with an event payload. Enumerates all possible [`Payload`s](Payload)
///
/// Parse with [`Event::parse`] or parse the whole http request your server receives with [`Payload::parse_http`]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    /// Channel Update V1 Event
    ChannelUpdateV1(Payload<channel::ChannelUpdateV1>),
    /// Channel Follow V1 Event
    ChannelFollowV1(Payload<channel::ChannelFollowV1>),
    /// Channel Subscribe V1 Event
    ChannelSubscribeV1(Payload<channel::ChannelSubscribeV1>),
    /// Channel Cheer V1 Event
    ChannelCheerV1(Payload<channel::ChannelCheerV1>),
    /// Channel Ban V1 Event
    ChannelBanV1(Payload<channel::ChannelBanV1>),
    /// Channel Unban V1 Event
    ChannelUnbanV1(Payload<channel::ChannelUnbanV1>),
    /// Channel Points Custom Reward Add V1 Event
    ChannelPointsCustomRewardAddV1(Payload<channel::ChannelPointsCustomRewardAddV1>),
    /// Channel Points Custom Reward Update V1 Event
    ChannelPointsCustomRewardUpdateV1(Payload<channel::ChannelPointsCustomRewardUpdateV1>),
    /// Channel Points Custom Reward Remove V1 Event
    ChannelPointsCustomRewardRemoveV1(Payload<channel::ChannelPointsCustomRewardRemoveV1>),
    /// Channel Points Custom Reward Redemption Add V1 Event
    ChannelPointsCustomRewardRedemptionAddV1(
        Payload<channel::ChannelPointsCustomRewardRedemptionAddV1>,
    ),
    /// Channel Points Custom Reward Redemption Update V1 Event
    ChannelPointsCustomRewardRedemptionUpdateV1(
        Payload<channel::ChannelPointsCustomRewardRedemptionUpdateV1>,
    ),
    /// Channel Poll Begin V1 Event
    ChannelPollBeginV1(Payload<channel::ChannelPollBeginV1>),
    /// Channel Poll Progress V1 Event
    ChannelPollProgressV1(Payload<channel::ChannelPollProgressV1>),
    /// Channel Poll End V1 Event
    ChannelPollEndV1(Payload<channel::ChannelPollEndV1>),
    /// Channel Prediction Begin V1 Event
    ChannelPredictionBeginV1(Payload<channel::ChannelPredictionBeginV1>),
    /// Channel Prediction Progress V1 Event
    ChannelPredictionProgressV1(Payload<channel::ChannelPredictionProgressV1>),
    /// Channel Prediction Lock V1 Event
    ChannelPredictionLockV1(Payload<channel::ChannelPredictionLockV1>),
    /// Channel Prediction End V1 Event
    ChannelPredictionEndV1(Payload<channel::ChannelPredictionEndV1>),
    /// Channel Goal Begin V1 Event
    ChannelGoalBeginV1(Payload<channel::ChannelGoalBeginV1>),
    /// Channel Goal Progress V1 Event
    ChannelGoalProgressV1(Payload<channel::ChannelGoalProgressV1>),
    /// Channel Goal End V1 Event
    ChannelGoalEndV1(Payload<channel::ChannelGoalEndV1>),
    /// Channel Hype Train Begin V1 Event
    ChannelHypeTrainBeginV1(Payload<channel::ChannelHypeTrainBeginV1>),
    /// Channel Hype Train Progress V1 Event
    ChannelHypeTrainProgressV1(Payload<channel::ChannelHypeTrainProgressV1>),
    /// Channel Hype Train End V1 Event
    ChannelHypeTrainEndV1(Payload<channel::ChannelHypeTrainEndV1>),
    /// StreamOnline V1 Event
    StreamOnlineV1(Payload<stream::StreamOnlineV1>),
    /// StreamOffline V1 Event
    StreamOfflineV1(Payload<stream::StreamOfflineV1>),
    /// User Update V1 Event
    UserUpdateV1(Payload<user::UserUpdateV1>),
    /// User Authorization Grant V1 Event
    UserAuthorizationGrantV1(Payload<user::UserAuthorizationGrantV1>),
    /// User Authorization Revoke V1 Event
    UserAuthorizationRevokeV1(Payload<user::UserAuthorizationRevokeV1>),
    /// Channel Raid V1 Event
    ChannelRaidV1(Payload<channel::ChannelRaidV1>),
    /// Channel Subscription End V1 Event
    ChannelSubscriptionEndV1(Payload<channel::ChannelSubscriptionEndV1>),
    /// Channel Subscription Gift V1 Event
    ChannelSubscriptionGiftV1(Payload<channel::ChannelSubscriptionGiftV1>),
    /// Channel Subscription Message V1 Event
    ChannelSubscriptionMessageV1(Payload<channel::ChannelSubscriptionMessageV1>),
}

impl Event {
    /// Parse string slice as an [`Event`]. Consider using [`Event::parse_http`] instead.
    pub fn parse(source: &str) -> Result<Event, PayloadParseError> {
        let (version, ty, message_type) =
            get_version_event_type_and_message_type_from_text(source)?;
        Self::parse_request(version, &ty, message_type, source.as_bytes().into())
    }

    /// Returns `true` if the message in the [`Payload`] is [`Revocation`].
    ///
    /// [`Revocation`]: Message::Revocation
    pub fn is_notification(&self) -> bool { is_thing!(self, Notification) }

    /// Returns `true` if the message in the [`Payload`] is [`Revocation`].
    ///
    /// [`Revocation`]: Message::Revocation
    pub fn is_revocation(&self) -> bool { is_thing!(self, Revocation) }

    /// Returns `true` if the message in the [`Payload`] is [`Revocation`].
    ///
    /// [`Revocation`]: Message::Revocation
    pub fn is_verification_request(&self) -> bool { is_thing!(self, VerificationRequest) }

    /// If this event is a [`VerificationRequest`], return the [`VerificationRequest`] message, including the message.
    #[rustfmt::skip]
    pub fn get_verification_request(&self) -> Option<&VerificationRequest> {
        // FIXME: Replace with proc_macro if a proc_macro crate is ever made
        match &self {
            Event::ChannelUpdateV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelFollowV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelSubscribeV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelCheerV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelBanV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelUnbanV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPointsCustomRewardAddV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPointsCustomRewardUpdateV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPointsCustomRewardRemoveV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPointsCustomRewardRedemptionAddV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPointsCustomRewardRedemptionUpdateV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPollBeginV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPollProgressV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPollEndV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPredictionBeginV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPredictionProgressV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPredictionLockV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelPredictionEndV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelGoalBeginV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelGoalProgressV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelGoalEndV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelHypeTrainBeginV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelHypeTrainProgressV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelHypeTrainEndV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::StreamOnlineV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::StreamOfflineV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::UserUpdateV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::UserAuthorizationGrantV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::UserAuthorizationRevokeV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelRaidV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelSubscriptionEndV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelSubscriptionGiftV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            Event::ChannelSubscriptionMessageV1(Payload { message: Message::VerificationRequest(v), ..}) => Some(v),
            _ => None,
        }
    }

    /// Make a [`EventSubSubscription`] from this notification.
    pub fn subscription(&self) -> Result<EventSubSubscription, serde_json::Error> {
        macro_rules! match_event {
        ($($module:ident::$event:ident);* $(;)?) => {{
            match &self {
                $(
                    Event::$event(notif) => Ok({
                        let self::Payload {subscription, ..} = notif; // FIXME: Use @ pattern-binding, currently stable

                        EventSubSubscription {
                        cost: subscription.cost,
                        condition: subscription.condition.condition()?,
                        created_at: subscription.created_at.clone(),
                        id: subscription.id.clone(),
                        status: subscription.status.clone(),
                        transport: subscription.transport.clone(),
                        type_: notif.get_event_type(),
                        version: notif.get_event_version().to_owned(),
                    }}),
                )*
            }
        }}
    }

        match_event!(
            channel::ChannelUpdateV1;
            channel::ChannelFollowV1;
            channel::ChannelSubscribeV1;
            channel::ChannelCheerV1;
            channel::ChannelBanV1;
            channel::ChannelUnbanV1;
            channel::ChannelPointsCustomRewardAddV1;
            channel::ChannelPointsCustomRewardUpdateV1;
            channel::ChannelPointsCustomRewardRemoveV1;
            channel::ChannelPointsCustomRewardRedemptionAddV1;
            channel::ChannelPointsCustomRewardRedemptionUpdateV1;
            channel::ChannelPollBeginV1;
            channel::ChannelPollProgressV1;
            channel::ChannelPollEndV1;
            channel::ChannelPredictionBeginV1;
            channel::ChannelPredictionProgressV1;
            channel::ChannelPredictionLockV1;
            channel::ChannelPredictionEndV1;
            channel::ChannelRaidV1;
            channel::ChannelSubscriptionEndV1;
            channel::ChannelSubscriptionGiftV1;
            channel::ChannelSubscriptionMessageV1;
            channel::ChannelGoalBeginV1;
            channel::ChannelGoalProgressV1;
            channel::ChannelGoalEndV1;
            channel::ChannelHypeTrainBeginV1;
            channel::ChannelHypeTrainProgressV1;
            channel::ChannelHypeTrainEndV1;
            stream::StreamOnlineV1;
            stream::StreamOfflineV1;
            user::UserUpdateV1;
            user::UserAuthorizationGrantV1;
            user::UserAuthorizationRevokeV1;
        )
    }

    /// Verify that this event is authentic using `HMAC-SHA256`.
    ///
    /// HMAC key is `secret`, HMAC message is a concatenation of `Twitch-Eventsub-Message-Id` header, `Twitch-Eventsub-Message-Timestamp` header and the request body.
    /// HMAC signature is `Twitch-Eventsub-Message-Signature` header.
    #[cfg(feature = "hmac")]
    #[cfg_attr(nightly, doc(cfg(feature = "hmac")))]
    #[must_use]
    pub fn verify_payload<B>(request: &http::Request<B>, secret: &[u8]) -> bool
    where B: AsRef<[u8]> {
        use crypto_hmac::{Hmac, Mac, NewMac};

        fn message_and_signature<B>(request: &http::Request<B>) -> Option<(Vec<u8>, Vec<u8>)>
        where B: AsRef<[u8]> {
            static SHA_HEADER: &str = "sha256=";

            let id = request
                .headers()
                .get("Twitch-Eventsub-Message-Id")?
                .as_bytes();
            let timestamp = request
                .headers()
                .get("Twitch-Eventsub-Message-Timestamp")?
                .as_bytes();
            let body = request.body().as_ref();

            let mut message = Vec::with_capacity(id.len() + timestamp.len() + body.len());
            message.extend_from_slice(id);
            message.extend_from_slice(timestamp);
            message.extend_from_slice(body);

            let signature = request
                .headers()
                .get("Twitch-Eventsub-Message-Signature")?
                .to_str()
                .ok()?;
            if !signature.starts_with(&SHA_HEADER) {
                return None;
            }
            let signature = signature.split_at(SHA_HEADER.len()).1;
            if signature.len() % 2 == 0 {
                // Convert signature to [u8] from hex digits
                // Hex decode inspired by https://stackoverflow.com/a/52992629
                let signature = ((0..signature.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&signature[i..i + 2], 16))
                    .collect::<Result<Vec<u8>, _>>())
                .ok()?;

                Some((message, signature))
            } else {
                None
            }
        }

        if let Some((message, signature)) = message_and_signature(request) {
            let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret).expect("");
            mac.update(&message);
            mac.verify(&signature).is_ok()
        } else {
            false
        }
    }
}

/// Helper function to get version and type of event from text.
#[allow(clippy::type_complexity)]
fn get_version_event_type_and_message_type_from_text(
    source: &str,
) -> Result<(Cow<'_, str>, EventType, Cow<'_, [u8]>), PayloadParseError> {
    #[derive(Deserialize)]
    struct IEventSubscripionInformation {
        // condition: serde_json::Value,
        // created_at: types::Timestamp,
        // status: Status,
        // cost: usize,
        // id: types::EventSubId,
        // transport: TransportResponse,
        #[serde(rename = "type")]
        type_: EventType,
        version: String,
    }
    #[derive(Deserialize)]
    struct IEvent {
        subscription: IEventSubscripionInformation,
        challenge: Option<Empty>,
        event: Option<Empty>,
    }

    #[derive(Deserialize)]
    struct Empty {}

    let IEvent {
        subscription,
        challenge,
        event,
    } = parse_json(source, false)?;
    // FIXME: A visitor is really what we want.
    if event.is_some() {
        Ok((
            subscription.version.into(),
            subscription.type_,
            Cow::Borrowed(b"notification"),
        ))
    } else if challenge.is_some() {
        Ok((
            subscription.version.into(),
            subscription.type_,
            Cow::Borrowed(b"webhook_callback_verification"),
        ))
    } else {
        Ok((
            subscription.version.into(),
            subscription.type_,
            Cow::Borrowed(b"revocation"),
        ))
    }
}

/// Helper function to get version and type of event from http.
#[allow(clippy::type_complexity)]
fn get_version_event_type_and_message_type_from_http<B>(
    request: &http::Request<B>,
) -> Result<(Cow<'_, str>, EventType, Cow<'_, [u8]>), PayloadParseError>
where B: AsRef<[u8]> {
    use serde::de::IntoDeserializer;
    match (
        request
            .headers()
            .get("Twitch-Eventsub-Subscription-Type")
            .map(|v| v.as_bytes())
            .map(std::str::from_utf8)
            .transpose()?,
        request
            .headers()
            .get("Twitch-Eventsub-Subscription-Version")
            .map(|v| v.as_bytes())
            .map(std::str::from_utf8)
            .transpose()?,
        request
            .headers()
            .get("Twitch-Eventsub-Message-Type")
            .map(|v| v.as_bytes()),
    ) {
        (Some(ty), Some(version), Some(message_type)) => Ok((
            version.into(),
            EventType::deserialize(ty.into_deserializer()).map_err(
                |_: serde::de::value::Error| PayloadParseError::UnknownEventType(ty.to_owned()),
            )?,
            message_type.into(),
        )),
        (..) => Err(PayloadParseError::MalformedEvent),
    }
}

impl Event {
    /// Parse a http payload as an [`Event`]
    pub fn parse_http<B>(request: &http::Request<B>) -> Result<Event, PayloadParseError>
    where B: AsRef<[u8]> {
        let (version, ty, message_type) =
            get_version_event_type_and_message_type_from_http(request)?;
        let source = request.body().as_ref().into();
        Self::parse_request(version, &ty, message_type, source)
    }

    /// Parse a string slice as an [`Event`]. You should not use this, instead, use [`Event::parse_http`] or [`Event::parse`].
    #[doc(hidden)]
    pub fn parse_request<'a>(
        version: Cow<'a, str>,
        event_type: &'a EventType,
        message_type: Cow<'a, [u8]>,
        source: Cow<'a, [u8]>,
    ) -> Result<Event, PayloadParseError> {
        /// Match on all defined eventsub types.
        ///
        /// If this is not done, we'd get a much worse error message.
        macro_rules! match_event {
            ($($module:ident::$event:ident);* $(;)?) => {{

                #[deny(unreachable_patterns)]
                match (version.as_ref(), event_type) {
                    $(  (<$module::$event as EventSubscription>::VERSION, &<$module::$event as EventSubscription>::EVENT_TYPE) => {
                        Event::$event(Payload::parse_request(message_type, source)?)
                    }  )*
                    (v, e) => return Err(PayloadParseError::UnimplementedEvent{version: v.to_owned(), event_type: e.clone()})
                }
            }}
        }

        Ok(match_event! {
            channel::ChannelUpdateV1;
            channel::ChannelFollowV1;
            channel::ChannelSubscribeV1;
            channel::ChannelCheerV1;
            channel::ChannelBanV1;
            channel::ChannelUnbanV1;
            channel::ChannelPointsCustomRewardAddV1;
            channel::ChannelPointsCustomRewardUpdateV1;
            channel::ChannelPointsCustomRewardRemoveV1;
            channel::ChannelPointsCustomRewardRedemptionAddV1;
            channel::ChannelPointsCustomRewardRedemptionUpdateV1;
            channel::ChannelPollBeginV1;
            channel::ChannelPollProgressV1;
            channel::ChannelPollEndV1;
            channel::ChannelPredictionBeginV1;
            channel::ChannelPredictionProgressV1;
            channel::ChannelPredictionLockV1;
            channel::ChannelPredictionEndV1;
            channel::ChannelRaidV1;
            channel::ChannelSubscriptionEndV1;
            channel::ChannelSubscriptionGiftV1;
            channel::ChannelSubscriptionMessageV1;
            channel::ChannelGoalBeginV1;
            channel::ChannelGoalProgressV1;
            channel::ChannelGoalEndV1;
            channel::ChannelHypeTrainBeginV1;
            channel::ChannelHypeTrainProgressV1;
            channel::ChannelHypeTrainEndV1;
            stream::StreamOnlineV1;
            stream::StreamOfflineV1;
            user::UserUpdateV1;
            user::UserAuthorizationGrantV1;
            user::UserAuthorizationRevokeV1;
        })
    }
}
