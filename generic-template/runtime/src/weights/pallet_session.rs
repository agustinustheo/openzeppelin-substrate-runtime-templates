// //! Autogenerated weights for `pallet_session`
// //!
// //! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
// //! DATE: 2024-11-03, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
// //! WORST CASE MAP SIZE: `1000000`
// //! HOSTNAME: `ip-172-31-15-118`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
// //! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// // Executed Command:
// // target/release/generic-template-node
// // benchmark
// // pallet
// // --steps=50
// // --repeat=20
// // --extrinsic=*
// // --wasm-execution=compiled
// // --heap-pages=4096
// // --json-file=benchmarking/results/results-pallet_session.json
// // --pallet=pallet_session
// // --chain=dev
// // --output=benchmarking/new-benchmarks/pallet_session.rs

// // #![cfg_attr(rustfmt, rustfmt_skip)]
// // #![allow(unused_parens)]
// // #![allow(unused_imports)]
// // #![allow(missing_docs)]

// // use frame_support::{traits::Get, weights::Weight};
// // use core::marker::PhantomData;

// /// Weight functions for `pallet_session`.
// pub struct WeightInfo<T>(PhantomData<T>);
// impl<T: frame_system::Config> pallet_session::WeightInfo for WeightInfo<T> {
// 	/// Storage: `Session::NextKeys` (r:1 w:1)
// 	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
// 	/// Storage: `Session::KeyOwner` (r:1 w:1)
// 	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
// 	fn set_keys() -> Weight {
// 		// Proof Size summary in bytes:
// 		//  Measured:  `298`
// 		//  Estimated: `3763`
// 		// Minimum execution time: 21_646_000 picoseconds.
// 		Weight::from_parts(21_946_000, 0)
// 			.saturating_add(Weight::from_parts(0, 3763))
// 			.saturating_add(T::DbWeight::get().reads(2))
// 			.saturating_add(T::DbWeight::get().writes(2))
// 	}
// 	/// Storage: `Session::NextKeys` (r:1 w:1)
// 	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
// 	/// Storage: `Session::KeyOwner` (r:0 w:1)
// 	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
// 	fn purge_keys() -> Weight {
// 		// Proof Size summary in bytes:
// 		//  Measured:  `280`
// 		//  Estimated: `3745`
// 		// Minimum execution time: 15_321_000 picoseconds.
// 		Weight::from_parts(15_898_000, 0)
// 			.saturating_add(Weight::from_parts(0, 3745))
// 			.saturating_add(T::DbWeight::get().reads(1))
// 			.saturating_add(T::DbWeight::get().writes(2))
// 	}
// }
