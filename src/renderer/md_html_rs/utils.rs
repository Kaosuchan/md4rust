#[inline]
#[allow(clippy::manual_range_contains)]
pub fn isdigit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

#[inline]
#[allow(clippy::manual_range_contains)]
pub fn islower(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z'
}

#[inline]
#[allow(clippy::manual_range_contains)]
pub fn isupper(ch: u8) -> bool {
    b'A' <= ch && ch <= b'Z'
}

#[inline]
pub fn isalnum(ch: u8) -> bool {
    isdigit(ch) || islower(ch) || isupper(ch)
}

#[inline]
pub fn hex_val(ch: u8) -> u8 {
    if isdigit(ch) {
        ch - b'0'
    } else if isupper(ch) {
        ch - b'A' + 10
    } else if islower(ch) {
        ch - b'a' + 10
    } else {
        unreachable!()
    }
}
