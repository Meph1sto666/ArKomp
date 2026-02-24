use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Event {
    OnRetreat {
        to: String,
        from: String,
    },
    SetSkin {
        to: String,
        skin: String,
    },
    SetAnimation {
        to: String,
        ani: String,
    },
    OnAnimationChange {
        to: String,
        from: String,
        ani: String,
    },
    MoveTo {
        to: String,
        position: (f32, f32),
    },
    Resize {
        to: String,
        scale: f32,
    },
    SetFacingDirection {
        to: String,
        direction: String,
    },
    SetPosition {
        to: String,
        position: (f32, f32),
    },

    CustomEvent {
        from: String,
        to: String,
        payload: String,
    },
}

impl Event {
    pub fn to(&self) -> &str {
        match self {
            Event::OnRetreat { to, .. } => to,
            Event::SetSkin { to, .. } => to,
            Event::SetAnimation { to, .. } => to,
            Event::MoveTo { to, .. } => to,
            Event::CustomEvent { to, .. } => to,
            Event::Resize { to, .. } => to,
            Event::SetFacingDirection { to, .. } => to,
            Event::SetPosition { to, .. } => to,
            Event::OnAnimationChange { to, .. } => to,
        }
    }
    pub fn from(&self) -> &str {
        match self {
            Event::OnRetreat { from, .. } => from,
            Event::SetSkin { .. } => "",
            Event::SetAnimation { .. } => "",
            Event::OnAnimationChange { from, .. } => from,
            Event::MoveTo { .. } => "",
            Event::Resize { .. } => "",
            Event::SetFacingDirection { .. } => "",
            Event::SetPosition { .. } => "",
            Event::CustomEvent { from, .. } => from,
        }
    }
}
