use crate::{Pointer, PointerType, Position, Read, ReadType, StaticSize, Write, WriteType};
use bitvec::prelude::*;
use num::ToPrimitive;
use std::convert::TryInto;

impl<'a> ReadType<'a> for BitVec<u8, Lsb0> {
	type ReadType = Pointer<&'a BitSlice<u8, Lsb0>>;
}

impl<'a> Read<'a> for &'a BitSlice<u8, Lsb0> {
	type Output = &'a BitSlice<u8, Lsb0>;
	fn read(bytes: &'a [u8], position: Position<Self>) -> Self::Output {
		let position = position.cast();
		let bit_len = PointerType::read(bytes, position);
		let bit_len = bit_len.to_usize().unwrap();
		let byte_len = (bit_len / 8) + if bit_len % 8 == 0 { 0 } else { 1 };
		let position = *position + PointerType::STATIC_SIZE;
		let position = position as usize;
		let slice = &bytes[position..position + byte_len];
		let bit_slice = BitSlice::from_slice(slice);
		&bit_slice[..bit_len]
	}
}

impl WriteType for BitVec<u8, Lsb0> {
	type WriteType = Position<BitVec<u8, Lsb0>>;
}

impl Write for BitVec<u8, Lsb0> {
	type Output = BitVec<u8, Lsb0>;
	fn write(&self, writer: &mut crate::Writer) -> Position<Self::Output> {
		let position = writer.position();
		let bit_len: PointerType = self.len().try_into().unwrap();
		writer.write(&bit_len);
		writer.write_raw::<[u8]>(self.as_raw_slice());
		position
	}
}
