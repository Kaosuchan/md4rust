static NEED_HTML_ESC_FLAG: u8 = 0x1;
static NEED_URL_ESC_FLAG: u8 = 0x2;

static ESCAPE_MAP: [u8; 256] = [
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 3, 0, 0, 0, 3, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 2, 2, 2, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
];

#[inline]
pub fn need_html_esc(ch: u8) -> bool {
    ESCAPE_MAP[ch as usize] & NEED_HTML_ESC_FLAG != 0
}

#[inline]
pub fn need_url_esc(ch: u8) -> bool {
    ESCAPE_MAP[ch as usize] & NEED_URL_ESC_FLAG != 0
}

#[test]
fn make_escape_map() {
    let mut a = [0u8; 256];
    for i in b"\"&<>".to_owned() {
        a[i as usize] |= NEED_HTML_ESC_FLAG;
    }
    for i in 0..256 {
        a[i as usize] |= NEED_URL_ESC_FLAG;
    }
    for i in b'0'..=b'9' {
        a[i as usize] ^= NEED_URL_ESC_FLAG;
    }
    for i in b'A'..=b'Z' {
        a[i as usize] ^= NEED_URL_ESC_FLAG;
    }
    for i in b'a'..=b'z' {
        a[i as usize] ^= NEED_URL_ESC_FLAG;
    }
    for i in b"~-_.+!*(),%#@?=;:/$".to_owned() {
        a[i as usize] ^= NEED_URL_ESC_FLAG;
    }
    print!("{:?}", a);
}
