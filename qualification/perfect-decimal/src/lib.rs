#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![doc = "Independent Perfect Qualification consumer of the pinned Perfect Decimal M1 public API."]

// The consumer intentionally exposes no replacement Decimal implementation.
// All mathematical reference calculations live independently in tests and
// retained Python standard-library oracle vectors.
