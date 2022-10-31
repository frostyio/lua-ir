pub struct Reader(Vec<u8>, usize); // (buffer, index)

impl Reader {
	pub fn from(buff: &[u8]) -> Self {
		Self(buff.to_vec(), 0)
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	pub fn byte(&mut self) -> u8 {
		self.1 += 1;
		return self.0[self.1 - 1];
	}

	pub fn bytes(&mut self, n: usize) -> Vec<u8> {
		self.1 += n;
		self.0[self.1 - n..self.1].to_vec()
	}

	pub fn short(&mut self) -> u16 {
		(self.byte() as u16) + ((self.byte() as u16) << 8)
	}

	#[inline]
	pub fn unsigned32(&mut self) -> u32 {
		let bytes = self.bytes(4);
		u32::from_le_bytes(bytes.try_into().unwrap())
	}

	#[inline]
	pub fn unsigned64(&mut self) -> u64 {
		let bytes = self.bytes(8);
		u64::from_le_bytes(bytes.try_into().unwrap())
	}

	#[inline]
	pub fn int(&mut self, n: usize) -> u64 {
		match n {
			4 => self.unsigned32() as u64,
			8 => self.unsigned64(),
			_ => unimplemented!(),
		}
	}

	pub fn number(&mut self, int: usize) -> f64 {
		let a = self.int(int) as u64;
		let b = self.int(int) as u64;
		f64::from_bits((b << 32) | a)
	}

	pub fn string(&mut self, size_t: u8) -> String {
		let str_size = self.int(size_t as usize);
		let mut str = self.bytes(str_size as usize).to_vec();
		str.pop(); // remove null character
		String::from_utf8(str).expect("invalid string")
	}
}

// pub struct Reader(Vec<u8>, usize);

// impl Reader {
// 	pub fn from(buffer: &[u8]) -> Self {
// 		Self(buffer.to_vec(), 0)
// 	}

// 	pub fn as_bytes(&self) -> &[u8] {
// 		&self.0
// 	}

// 	#[inline]
// 	pub fn byte(&mut self) -> u8 {
// 		let v = &self.0[self.1];
// 		self.1 += 1;
// 		*v
// 	}

// 	#[inline]
// 	pub fn bytes(&mut self, n: usize) -> Vec<u8> {
// 		let v = &self.0[self.1..self.1 + n];
// 		self.1 += n;
// 		v.to_vec()
// 	}

// 	#[inline]
// 	pub fn int(&mut self, n: usize) -> u32 {
// 		let bytes = self.bytes(n);
// 		let mut sum: u32 = 0;
// 		for i in (0..n - 1).rev() {
// 			sum = (sum << 8) + (bytes[i] as u32);
// 		}
// 		sum
// 	}

// 	#[inline]
// 	pub fn number(&mut self, int: usize) -> f64 {
// 		let a = self.int(int) as u64;
// 		let b = self.int(int) as u64;
// 		f64::from_bits((b << 32) | a)
// 	}

// 	#[inline]
// 	pub fn string(&mut self, size_t: u8) -> String {
// 		let str_size = self.int(size_t as usize);
// 		let mut str = self.bytes(str_size as usize).to_vec();
// 		str.pop(); // remove nul character
// 		String::from_utf8(str).expect("invalid string")
// 	}
// }
