use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Event {
    Retreat { op_id: String },
    SetSkin { op_id: String, skin: String },
    SetAnimation { op_id: String, ani: String },
    MoveTo { op_id: String, pos: (f32, f32) },
    Sleep { op_id: String },
    Sit { op_id: String },
    Resize { op_id: String, scale: f32 },
    SetFacingDirection { op_id: String, direction: bool },
    SetPosition { op_id: String, position: (f32, f32) },

    CustomEvent { op_id: String, payload: String },
}

impl Event {
    pub fn operator_id(&self) -> &str {
        match self {
            Event::Retreat { op_id } => op_id,
            Event::SetSkin { op_id, .. } => op_id,
            Event::SetAnimation { op_id, .. } => op_id,
            Event::MoveTo { op_id, .. } => op_id,
            Event::Sleep { op_id } => op_id,
            Event::Sit { op_id } => op_id,
            Event::CustomEvent { op_id, .. } => op_id,
            Event::Resize { op_id, .. } => op_id,
            Event::SetFacingDirection { op_id, .. } => op_id,
            Event::SetPosition { op_id, .. } => op_id,
        }
    }
}
