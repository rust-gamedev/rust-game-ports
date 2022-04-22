pub use crate::prelude::*;

// In order to respect the source constraint (see GameState#tick()), we need to store an
// Option<VirtualKeyCode>; there are two options:
//
// - directly store Option<VirtualKeyCode>;
// - wrap it.
//
// The first also works, however, the None case has ambiguos semantics, as it can be interpreted
// as both "no resource in the storage" or "None resource in the storage".
pub struct VirtualKeyCodeR(pub Option<VirtualKeyCode>);
