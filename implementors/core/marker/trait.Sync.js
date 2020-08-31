(function() {var implementors = {};
implementors["twitch_api2"] = [{"text":"impl&lt;'a, C&gt; Sync for HelixClient&lt;'a, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, C&gt; Sync for TMIClient&lt;'a, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, C&gt; Sync for TwitchClient&lt;'a, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;R, D&gt; Sync for Response&lt;R, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for Pagination","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for RequestError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl Sync for ChannelInformation","synthetic":true,"types":[]},{"text":"impl Sync for GetChannelInformationRequest","synthetic":true,"types":[]},{"text":"impl Sync for ModifyChannelInformationBody","synthetic":true,"types":[]},{"text":"impl Sync for ModifyChannelInformationRequest","synthetic":true,"types":[]},{"text":"impl Sync for StartCommercial","synthetic":true,"types":[]},{"text":"impl Sync for StartCommercialBody","synthetic":true,"types":[]},{"text":"impl Sync for StartCommercialRequest","synthetic":true,"types":[]},{"text":"impl Sync for ModifyChannelInformation","synthetic":true,"types":[]},{"text":"impl Sync for CommercialLength","synthetic":true,"types":[]},{"text":"impl Sync for CommercialLengthParseError","synthetic":true,"types":[]},{"text":"impl Sync for Clip","synthetic":true,"types":[]},{"text":"impl Sync for GetClipsRequest","synthetic":true,"types":[]},{"text":"impl Sync for CheckAutoModStatus","synthetic":true,"types":[]},{"text":"impl Sync for CheckAutoModStatusBody","synthetic":true,"types":[]},{"text":"impl Sync for CheckAutoModStatusRequest","synthetic":true,"types":[]},{"text":"impl Sync for BannedEvents","synthetic":true,"types":[]},{"text":"impl Sync for GetBannedEventsRequest","synthetic":true,"types":[]},{"text":"impl Sync for BannedUsers","synthetic":true,"types":[]},{"text":"impl Sync for GetBannedUsersRequest","synthetic":true,"types":[]},{"text":"impl Sync for GetModeratorEventsRequest","synthetic":true,"types":[]},{"text":"impl Sync for ModeratorEvents","synthetic":true,"types":[]},{"text":"impl Sync for GetModeratorsRequest","synthetic":true,"types":[]},{"text":"impl Sync for Moderators","synthetic":true,"types":[]},{"text":"impl Sync for GetStreamsRequest","synthetic":true,"types":[]},{"text":"impl Sync for Stream","synthetic":true,"types":[]},{"text":"impl Sync for StreamType","synthetic":true,"types":[]},{"text":"impl Sync for BroadcasterSubscriptions","synthetic":true,"types":[]},{"text":"impl Sync for GetBroadcasterSubscriptionsRequest","synthetic":true,"types":[]},{"text":"impl Sync for GetUsersRequest","synthetic":true,"types":[]},{"text":"impl Sync for User","synthetic":true,"types":[]},{"text":"impl Sync for GetChatters","synthetic":true,"types":[]},{"text":"impl Sync for Chatters","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for RequestError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl Sync for DummyHttpClient","synthetic":true,"types":[]},{"text":"impl Sync for SurfError","synthetic":true,"types":[]}];
implementors["twitch_oauth2"] = [{"text":"impl Sync for AppAccessToken","synthetic":true,"types":[]},{"text":"impl Sync for UserToken","synthetic":true,"types":[]},{"text":"impl Sync for ValidatedToken","synthetic":true,"types":[]},{"text":"impl Sync for Scope","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl&lt;EF, TT&gt; Sync for TwitchTokenResponse&lt;EF, TT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;EF: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;TT: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for TwitchTokenErrorResponse","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for TokenError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for ValidationError&lt;RE&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;RE: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for RevokeTokenError&lt;RE&gt;","synthetic":true,"types":[]},{"text":"impl&lt;RE&gt; Sync for RefreshTokenError&lt;RE&gt;","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()