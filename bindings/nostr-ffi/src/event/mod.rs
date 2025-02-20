// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::ops::Deref;

use nostr::Event as EventSdk;

pub mod builder;
pub mod kind;

use self::kind::Kind;
use crate::error::Result;

pub struct Event {
    event: EventSdk,
}

impl From<EventSdk> for Event {
    fn from(event: EventSdk) -> Self {
        Self { event }
    }
}

impl Deref for Event {
    type Target = EventSdk;
    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

impl Event {
    pub fn pubkey(&self) -> String {
        self.event.pubkey.to_string()
    }

    pub fn kind(&self) -> Kind {
        self.event.kind.into()
    }

    pub fn content(&self) -> String {
        self.event.content.clone()
    }
}

impl Event {
    pub fn verify(&self) -> bool {
        self.event.verify().is_ok()
    }

    pub fn from_json(json: String) -> Result<Self> {
        Ok(Self {
            event: EventSdk::from_json(json)?,
        })
    }

    pub fn as_json(&self) -> Result<String> {
        Ok(self.event.as_json()?)
    }
}
