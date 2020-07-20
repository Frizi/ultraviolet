/// This module provides optional support for external libraries under feature flags.

#[cfg(feature = "approx")]
mod approx;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "bytemuck")]
mod bytemuck;
