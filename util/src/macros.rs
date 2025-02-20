// Copyright 2021 The Why Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Rust Bitcoin Library
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Macros to support Rust BIP-32 code (though could conceivably be used for other things)

/// gives a newtype array wrapper standard array traits
#[macro_export]
macro_rules! impl_array_newtype {
	($thing:ident, $ty:ty, $len:expr) => {
		impl $thing {
			#[inline]
			/// Converts the object to a raw pointer
			pub fn as_ptr(&self) -> *const $ty {
				let &$thing(ref dat) = self;
				dat.as_ptr()
			}

			#[inline]
			/// Converts the object to a mutable raw pointer
			pub fn as_mut_ptr(&mut self) -> *mut $ty {
				let &mut $thing(ref mut dat) = self;
				dat.as_mut_ptr()
			}

			#[inline]
			/// Returns the length of the object as an array
			pub fn len(&self) -> usize {
				$len
			}

			#[inline]
			/// Returns whether the object, as an array, is empty. Always false.
			pub fn is_empty(&self) -> bool {
				false
			}

			#[inline]
			/// Returns the underlying bytes.
			pub fn as_bytes(&self) -> &[$ty; $len] {
				&self.0
			}

			#[inline]
			/// Returns the underlying bytes.
			pub fn to_bytes(&self) -> [$ty; $len] {
				self.0.clone()
			}

			#[inline]
			/// Returns the underlying bytes.
			pub fn into_bytes(self) -> [$ty; $len] {
				self.0
			}
		}

		impl<'a> From<&'a [$ty]> for $thing {
			fn from(data: &'a [$ty]) -> $thing {
				assert_eq!(data.len(), $len);
				let mut ret = [0; $len];
				ret.copy_from_slice(&data[..]);
				$thing(ret)
			}
		}

		impl_index_newtype!($thing, $ty);

		impl PartialEq for $thing {
			#[inline]
			fn eq(&self, other: &$thing) -> bool {
				&self[..] == &other[..]
			}
		}

		impl Eq for $thing {}

		impl PartialOrd for $thing {
			#[inline]
			fn partial_cmp(&self, other: &$thing) -> Option<::std::cmp::Ordering> {
				Some(self.cmp(&other))
			}
		}

		impl Ord for $thing {
			#[inline]
			fn cmp(&self, other: &$thing) -> ::std::cmp::Ordering {
				// manually implement comparison to get little-endian ordering
				// (we need this for our numeric types; non-numeric ones shouldn't
				// be ordered anyway except to put them in BTrees or whatever, and
				// they don't care how we order as long as we're consisistent).
				for i in 0..$len {
					if self[$len - 1 - i] < other[$len - 1 - i] {
						return ::std::cmp::Ordering::Less;
					}
					if self[$len - 1 - i] > other[$len - 1 - i] {
						return ::std::cmp::Ordering::Greater;
					}
				}
				::std::cmp::Ordering::Equal
			}
		}

		#[cfg_attr(feature = "clippy", allow(expl_impl_clone_on_copy))] // we don't define the `struct`, we have to explicitly impl
		impl Clone for $thing {
			#[inline]
			fn clone(&self) -> $thing {
				$thing::from(&self[..])
			}
		}

		impl Copy for $thing {}

		impl ::std::hash::Hash for $thing {
			#[inline]
			fn hash<H>(&self, state: &mut H)
			where
				H: ::std::hash::Hasher,
			{
				(&self[..]).hash(state);
			}

			fn hash_slice<H>(data: &[$thing], state: &mut H)
			where
				H: ::std::hash::Hasher,
			{
				for d in data.iter() {
					(&d[..]).hash(state);
				}
			}
		}
	};
}

/// gives a newtype array wrapper serialization and deserialization methods
#[macro_export]
macro_rules! impl_array_newtype_encodable {
	($thing:ident, $ty:ty, $len:expr) => {
		#[cfg(feature = "serde")]
		impl<'de> $crate::serde::Deserialize<'de> for $thing {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: $crate::serde::Deserializer<'de>,
			{
				use $crate::std::fmt::{self, Formatter};

				struct Visitor;
				impl<'de> $crate::serde::de::Visitor<'de> for Visitor {
					type Value = $thing;

					fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
						formatter.write_str("a fixed size array")
					}

					#[inline]
					fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
					where
						A: $crate::serde::de::SeqAccess<'de>,
					{
						let mut ret: [$ty; $len] = [0; $len];
						for item in ret.iter_mut() {
							*item = match seq.next_element()? {
								Some(c) => c,
								None => {
									return Err($crate::serde::de::Error::custom("end of stream"));
								}
							};
						}
						Ok($thing(ret))
					}
				}

				deserializer.deserialize_seq(Visitor)
			}
		}

		#[cfg(feature = "serde")]
		impl $crate::serde::Serialize for $thing {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: $crate::serde::Serializer,
			{
				let &$thing(ref dat) = self;
				(&dat[..]).serialize(serializer)
			}
		}
	};
}

/// gives a newtype array wrapper the Debug trait
#[macro_export]
macro_rules! impl_array_newtype_show {
	($thing:ident) => {
		impl ::std::fmt::Debug for $thing {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				write!(f, concat!(stringify!($thing), "({:?})"), &self[..])
			}
		}
	};
}

/// gives a newtype array wrapper Index traits
#[macro_export]
macro_rules! impl_index_newtype {
	($thing:ident, $ty:ty) => {
		impl ::std::ops::Index<usize> for $thing {
			type Output = $ty;

			#[inline]
			fn index(&self, index: usize) -> &$ty {
				let &$thing(ref dat) = self;
				&dat[index]
			}
		}
		impl ::std::ops::Index<::std::ops::Range<usize>> for $thing {
			type Output = [$ty];

			#[inline]
			fn index(&self, index: ::std::ops::Range<usize>) -> &[$ty] {
				&self.0[index]
			}
		}

		impl ::std::ops::Index<::std::ops::RangeTo<usize>> for $thing {
			type Output = [$ty];

			#[inline]
			fn index(&self, index: ::std::ops::RangeTo<usize>) -> &[$ty] {
				&self.0[index]
			}
		}

		impl ::std::ops::Index<::std::ops::RangeFrom<usize>> for $thing {
			type Output = [$ty];

			#[inline]
			fn index(&self, index: ::std::ops::RangeFrom<usize>) -> &[$ty] {
				&self.0[index]
			}
		}

		impl ::std::ops::Index<::std::ops::RangeFull> for $thing {
			type Output = [$ty];

			#[inline]
			fn index(&self, _: ::std::ops::RangeFull) -> &[$ty] {
				&self.0[..]
			}
		}
	};
}
