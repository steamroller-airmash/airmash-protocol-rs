use crate::enums::ServerMessageType;

/// Server banned message
#[derive(Clone, Debug)]
pub struct ServerMessage {
  // #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: ServerMessageType,
  // TODO: Make this a duration?
  pub duration: u32,
  pub text: String,
}
