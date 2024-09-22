// uses custom to/from methods
// While impl TryFrom<RequestPackage> for [u8; PACKAGE_SIZE] is possible
// package.into() is not straightforward enough
// I'd expect person to look for `to_bytes` method
pub trait ByteSerializable<const SIZE: usize> {
    fn to_bytes(&self) -> [u8; SIZE];
    fn from_bytes(raw_bytes: &[u8; SIZE]) -> Self;
}

pub fn read_field<const LENGTH: usize>(raw_bytes: &[u8], offset: usize) -> [u8; LENGTH] {
    raw_bytes[offset..(offset + LENGTH)]
        .try_into()
        .expect("lengthes always match")
}
