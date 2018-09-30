use types::Team;

/// A flag ID
#[cfg(feature = "specs")]
use specs::DenseVecStorage;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, From, Into, Constructor)]
#[cfg_attr(feature = "specs", derive(Component))]
pub struct Flag(pub Team);

wrapper_serde_decl!(Flag);
