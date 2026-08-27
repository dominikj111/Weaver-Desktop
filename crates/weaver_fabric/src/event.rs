//! Semantic event objects — data only, no closures or backend handles.
//!
//! Dispatch is always via event objects (local or global), never ad-hoc lambdas.
//! Events stay plain data (serializable in principle) so the contract can later
//! cross a process/network boundary, but serialization is deliberately not
//! introduced yet (see the design doc — remote rendering is deferred).

/// A semantic event emitted by a widget and handled by the application core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// Widget id that emitted the event.
    pub source: String,
    /// What happened.
    pub kind: EventKind,
}

/// The kinds of semantic events a widget can emit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    /// A user activated the widget (clicked a button, pressed a key).
    Activated,
    /// The widget's value changed.
    Changed,
}

impl Event {
    pub fn activated(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            kind: EventKind::Activated,
        }
    }

    pub fn changed(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            kind: EventKind::Changed,
        }
    }
}
