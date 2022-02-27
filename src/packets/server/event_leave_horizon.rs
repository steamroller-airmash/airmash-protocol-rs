use crate::enums::LeaveHorizonType;

/// Event for when a player goes beyond the event horizon.
///
/// This indicates that the server will stop sending updates about this plane
/// until it comes back within the event horizon.
#[derive(Copy, Clone, Debug)]
pub struct EventLeaveHorizon {
  // #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: LeaveHorizonType,
  /// This could be either a player or a mob
  /// TODO: Create Entity type
  pub id: u16,
}
