// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use nostr::url::Url;
use nostr::{Contact as ContactSdk, EventBuilder as EventBuilderSdk, Sha256Hash, Tag};

use super::kind::Kind;
use super::Event;
use crate::contact::Contact;
use crate::error::Result;
use crate::key::Keys;
use crate::metadata::Metadata;

pub struct EventBuilder {
    builder: EventBuilderSdk,
}

impl From<EventBuilderSdk> for EventBuilder {
    fn from(builder: EventBuilderSdk) -> Self {
        Self { builder }
    }
}

impl Deref for EventBuilder {
    type Target = EventBuilderSdk;
    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl EventBuilder {
    pub fn new(kind: Kind, content: String, tags: Vec<Vec<String>>) -> Self {
        let tags: Vec<Tag> = tags.into_iter().map(|tag| tag.into()).collect();
        Self {
            builder: EventBuilderSdk::new(kind.into(), &content, &tags),
        }
    }

    pub fn to_event(&self, keys: Arc<Keys>) -> Result<Arc<Event>> {
        let event = self.builder.clone().to_event(keys.deref())?;
        Ok(Arc::new(event.into()))
    }

    pub fn to_pow_event(&self, keys: Arc<Keys>, difficulty: u8) -> Result<Arc<Event>> {
        Ok(Arc::new(
            self.builder
                .clone()
                .to_pow_event(keys.deref(), difficulty)?
                .into(),
        ))
    }
}

impl EventBuilder {
    pub fn set_metadata(metadata: Arc<Metadata>) -> Result<Self> {
        Ok(Self {
            builder: EventBuilderSdk::set_metadata(metadata.as_ref().deref().clone())?,
        })
    }

    pub fn add_recommended_relay(url: String) -> Result<Self> {
        let url = Url::parse(&url)?;

        Ok(Self {
            builder: EventBuilderSdk::add_recommended_relay(&url),
        })
    }

    pub fn new_text_note(content: String, tags: Vec<Vec<String>>) -> Self {
        let tags: Vec<Tag> = tags.into_iter().map(|tag| tag.into()).collect();
        Self {
            builder: EventBuilderSdk::new_text_note(&content, &tags),
        }
    }

    pub fn set_contact_list(list: Vec<Arc<Contact>>) -> Self {
        let list: Vec<ContactSdk> = list
            .into_iter()
            .map(|c| c.as_ref().deref().clone())
            .collect();

        Self {
            builder: EventBuilderSdk::set_contact_list(list),
        }
    }

    /// Create encrypted direct msg event
    pub fn new_encrypted_direct_msg(
        sender_keys: Arc<Keys>,
        receiver_keys: Arc<Keys>,
        content: String,
    ) -> Result<Self> {
        Ok(Self {
            builder: EventBuilderSdk::new_encrypted_direct_msg(
                sender_keys.deref(),
                receiver_keys.deref(),
                &content,
            )?,
        })
    }

    /// Create delete event
    pub fn delete(ids: Vec<String>, reason: Option<String>) -> Result<Self> {
        let mut new_ids: Vec<Sha256Hash> = Vec::with_capacity(ids.len());

        for id in ids.into_iter() {
            new_ids.push(Sha256Hash::from_str(&id)?);
        }

        Ok(Self {
            builder: EventBuilderSdk::delete(new_ids, reason.as_deref()),
        })
    }

    /// Add reaction (like/upvote, dislike/downvote) to an event
    pub fn new_reaction(event: Arc<Event>, positive: bool) -> Self {
        Self {
            builder: EventBuilderSdk::new_reaction(event.deref(), positive),
        }
    }
}
