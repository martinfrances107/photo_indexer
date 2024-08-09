// Cantor Pairing.
//
// A hash function for two integers
//
// <https://en.wikipedia.org/wiki/Pairing_function>
#[cfg(feature = "ssr")]
#[inline]
pub(crate) const fn cantor_pair(k1: usize, k2: usize) -> usize {
    (k1 + k2) * (k1 + k2 + 1) / 2 + k2
}
