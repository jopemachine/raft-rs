use lazy_static::lazy_static;
use raft_proto::prelude::{ConfChange, ConfChangeV2, Message, Snapshot};
use std::sync::{Arc, RwLock};

lazy_static! {
    pub static ref CUSTOM_FORMATTER: RwLock<Arc<dyn CustomFormatter + Send + Sync>> =
        RwLock::new(Arc::new(DefaultFormatter));
}

use crate::prelude::Entry;

#[derive(Debug)]
pub enum Bytes {
    Prost(Vec<u8>),
    Protobuf(bytes::Bytes),
}

impl From<Vec<u8>> for Bytes {
    fn from(vec: Vec<u8>) -> Self {
        Bytes::Prost(vec)
    }
}

impl From<bytes::Bytes> for Bytes {
    fn from(bytes: bytes::Bytes) -> Self {
        Bytes::Protobuf(bytes)
    }
}

/// Sample Docs
pub trait CustomFormatter {
    /// Sample Docs
    fn format_entry_context(&self, v: &Bytes) -> String {
        format!("{:?}", v)
    }

    /// Sample Docs
    fn format_entry_data(&self, v: &Bytes) -> String {
        format!("{:?}", v)
    }

    /// Sample Docs
    fn format_confchangev2_context(&self, v: &Bytes) -> String {
        format!("{:?}", v)
    }

    /// Sample Docs
    fn format_confchange_context(&self, v: &Bytes) -> String {
        format!("{:?}", v)
    }

    /// Sample Docs
    fn format_message_context(&self, v: &Bytes) -> String {
        format!("{:?}", v)
    }

    /// Sample Docs
    fn format_snapshot_data(&self, v: &Bytes) -> String {
        format!("{:?}", v)
    }
}

struct DefaultFormatter;
impl CustomFormatter for DefaultFormatter {}

/// Sample Docs
pub fn format_entry(entry: &Entry) -> String {
    let formatter = CUSTOM_FORMATTER.read().unwrap();

    format!(
        "Entry {{ context: {context:}, data: {data:}, entry_type: {entry_type:?}, index: {index:}, sync_log: {sync_log:}, term: {term:} }}",
        data=formatter.format_entry_data(&entry.data.clone().into()),
        context=formatter.format_entry_context(&entry.context.clone().into()),
        entry_type=entry.get_entry_type(),
        index=entry.get_index(),
        sync_log=entry.get_sync_log(),
        term=entry.get_term(),
    )
}

/// Sample Docs
pub fn format_confchange(cc: &ConfChange) -> String {
    let formatter = CUSTOM_FORMATTER.read().unwrap();

    format!(
        "ConfChange {{ change_type: {change_type:?}, node_id: {node_id:}, context: {context:}, id: {id:} }}",
        change_type = cc.get_change_type(),
        node_id = cc.get_node_id(),
        id = cc.get_id(),
        context = formatter.format_confchange_context(&cc.context.clone().into())
    )
}

/// Sample Docs
pub fn format_confchangev2(cc: &ConfChangeV2) -> String {
    let formatter = CUSTOM_FORMATTER.read().unwrap();

    format!(
        "ConfChangeV2 {{ transition: {transition:?}, changes: {changes:?}, context: {context:} }}",
        transition = cc.transition,
        changes = cc.changes,
        context = formatter.format_confchangev2_context(&cc.context.clone().into())
    )
}

/// Sample Docs
pub fn format_snapshot(snapshot: &Snapshot) -> String {
    let formatter = CUSTOM_FORMATTER.read().unwrap();

    format!(
        "Snapshot {{ data: {data:}, metadata: {metadata:?} }}",
        data = formatter.format_snapshot_data(&snapshot.data.clone().into()),
        metadata = snapshot.metadata,
    )
}

/// Sample Docs
pub fn format_message(msg: &Message) -> String {
    let formatter = CUSTOM_FORMATTER.read().unwrap();

    format!(
        "Message {{ msg_type: {msg_type:?}, to: {to:}, from: {from:}, term: {term:}, log_term: {log_term:}, index: {index:}, entries: [{entries:}], commit: {commit:}, commit_term: {commit_term:}, snapshot: {snapshot:}, request_snapshot: {request_snapshot:}, reject: {reject:}, reject_hint: {reject_hint:}, context: {context:}, deprecated_priority: {deprecated_priority:}, priority: {priority:} }}",
        msg_type=msg.get_msg_type(),
        to=msg.get_to(),
        from=msg.get_from(),
        term=msg.get_term(),
        log_term=msg.get_log_term(),
        index=msg.get_index(),
        entries=msg.get_entries().iter().map(|e| format_entry(e)).collect::<Vec<String>>().join(", "),
        commit=msg.get_commit(),
        commit_term=msg.get_commit_term(),
        snapshot=format_snapshot(msg.get_snapshot()),
        request_snapshot=msg.get_request_snapshot(),
        reject=msg.get_reject(),
        reject_hint=msg.get_reject_hint(),
        context=formatter.format_message_context(&msg.context.clone().into()),
        deprecated_priority=msg.get_deprecated_priority(),
        priority=msg.get_priority(),
    )
}

/// Sample Docs
pub fn set_custom_formatter<D: CustomFormatter + 'static + Send + Sync>(formatter: D) {
    let formatter = Arc::new(formatter);
    let mut global_formatter = CUSTOM_FORMATTER.write().unwrap();
    *global_formatter = formatter;
}
