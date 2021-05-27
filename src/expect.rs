
use std::ops::Range;
use crate::*;
use std::fmt::Debug;

/// Types that are `Expect` can be constructed from a `Parser`.
pub trait Expect : Debug + Clone {

	/// Advances the position of some `Parser`, and returns `Ok(Self)` if successful and `Err(Unexpected<Self>)` if not.
	fn expect_from(parser: &mut Parser) -> Result<Self>
	where Self : Sized + Debug;
	

}
