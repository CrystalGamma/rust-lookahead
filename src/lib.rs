pub struct LookAhead<T, I: Iterator<T>> {
	iter: I,
	la: T
}

impl<T, I: Iterator<T>> LookAhead<T, I> {
	pub fn new(mut inner: I) -> Option<LookAhead<T, I>> {
		Some(LookAhead {
			la: match inner.next() {
				Some(x) => x,
				None => return None
			},
			iter: inner
		})
	}
	pub fn peek<'a>(&'a self) -> &'a T {
		&self.la
	}
	pub fn proceed(&mut self) -> bool {
		self.la = match self.iter.next() {
			Some(x) => x,
			None => return false
		};
		true
	}
}