use serde::{Deserialize, Serialize};
use seshat::{CrawlerCheckpoint, LoadConfig, Profile, SearchConfig};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "method")]
pub enum Message {
    InitEventIndex(InitEventIndex),
    LoadCheckpoints,
    IsEventIndexEmpty,
    CommitLiveEvents,
    AddEventToIndex { content: AddEventToIndex },
    AddCrawlerCheckpoint { content: AddHistoricEvents },
    AddHistoricEvents { content: AddHistoricEvents },
    RemoveCrawlerCheckpoint { content: AddHistoricEvents },
    SearchEventIndex { content: SearchEventIndex },
    LoadFileEvents { content: LoadConfig },
    DeleteEvent { content: DeleteEvent },
    GetStats,
    CloseEventIndex,
    DeleteEventIndex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitEventIndex {
    pub passphrase: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddEventToIndex {
    pub ev: Event,
    pub profile: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddHistoricEvents {
    pub checkpoint: Option<CrawlerCheckpoint>,
    pub old_checkpoint: Option<CrawlerCheckpoint>,
    pub events: Option<Vec<Events>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchEventIndex {
    pub term: String,
    pub config: SearchConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEvent {
    pub event_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pub event: Event,
    pub profile: Profile,
}

// remove duplication once specialization lands
// https://github.com/rust-lang/rust/issues/31844
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "m.room.message")]
    Message(EventMessage),
    #[serde(rename = "m.room.name")]
    Name(EventName),
    #[serde(rename = "m.room.topic")]
    Topic(EventTopic),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventMessage {
    pub event_id: String,
    pub sender: String,
    #[serde(rename = "origin_server_ts")]
    pub server_ts: i64,
    pub room_id: String,
    pub content: EventMessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventMessageContent {
    pub body: String,
    pub msgtype: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventName {
    pub event_id: String,
    pub sender: String,
    #[serde(rename = "origin_server_ts")]
    pub server_ts: i64,
    pub room_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTopic {
    pub event_id: String,
    pub sender: String,
    #[serde(rename = "origin_server_ts")]
    pub server_ts: i64,
    pub room_id: String,
    pub topic: String,
}
