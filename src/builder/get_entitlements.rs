use std::borrow::Cow;

#[cfg(feature = "http")]
use crate::http::CacheHttp;
use crate::internal::prelude::Result;
use crate::model::id::{EntitlementId, GuildId, SkuId, UserId};
use crate::model::monetization::Entitlement;

/// Builds a request to fetch active and ended [`Entitlement`]s.
///
/// This is a helper for [`Http::get_entitlements`] used via [`Entitlement::list`].
///
/// [`Http::get_entitlements`]: crate::http::Http::get_entitlements
#[derive(Clone, Debug, Default)]
#[must_use]
pub struct GetEntitlements<'a> {
    user_id: Option<UserId>,
    sku_ids: Option<Cow<'a, [SkuId]>>,
    before: Option<EntitlementId>,
    after: Option<EntitlementId>,
    limit: Option<u8>,
    guild_id: Option<GuildId>,
    exclude_ended: Option<bool>,
}

impl<'a> GetEntitlements<'a> {
    /// Filters the returned entitlements by the given [`UserId`].
    pub fn user_id(mut self, user_id: UserId) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Filters the returned entitlements by the given [`SkuId`]s.
    pub fn sku_ids(mut self, sku_ids: impl Into<Cow<'a, [SkuId]>>) -> Self {
        self.sku_ids = Some(sku_ids.into());
        self
    }

    /// Filters the returned entitlements to before the given [`EntitlementId`].
    pub fn before(mut self, before: EntitlementId) -> Self {
        self.before = Some(before);
        self
    }

    /// Filters the returned entitlements to after the given [`EntitlementId`].
    pub fn after(mut self, after: EntitlementId) -> Self {
        self.after = Some(after);
        self
    }

    /// Limits the number of entitlements that may be returned.
    ///
    /// This is limited to `0..=100`.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filters the returned entitlements by the given [`GuildId`].
    pub fn guild_id(mut self, guild_id: GuildId) -> Self {
        self.guild_id = Some(guild_id);
        self
    }

    /// Filters the returned entitlements to only active entitlements, if `true`.
    pub fn exclude_ended(mut self, exclude_ended: bool) -> Self {
        self.exclude_ended = Some(exclude_ended);
        self
    }
}

#[cfg(feature = "http")]
#[async_trait::async_trait]
impl super::Builder for GetEntitlements<'_> {
    type Context<'ctx> = ();
    type Built = Vec<Entitlement>;

    async fn execute(
        self,
        cache_http: impl CacheHttp,
        _: Self::Context<'_>,
    ) -> Result<Self::Built> {
        let http = cache_http.http();
        http.get_entitlements(
            self.user_id,
            self.sku_ids.as_deref(),
            self.before,
            self.after,
            self.limit,
            self.guild_id,
            self.exclude_ended,
        )
        .await
    }
}
