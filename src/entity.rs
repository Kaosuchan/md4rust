use std::os::raw::*;

mod sys {
    pub use crate::md4c_sys::entity::*;
}

pub fn entity_lookup_raw(name: &[u8]) -> Option<&[u32]> {
    let name_size = name.len();
    let name = name.as_ptr() as *const c_char;

    let entity;

    unsafe {
        #[cfg(target_pointer_width = "64")]
        {
            entity = sys::entity_lookup(name, name_size as u64);
        }
        #[cfg(target_pointer_width = "32")]
        {
            entity = sys::entity_lookup(name, name_size as u32);
        }

        if entity.is_null() {
            return None;
        }

        let codepoints = &(*entity).codepoints;
        let c2 = codepoints[1];

        if c2 == 0 {
            Some(&codepoints[0..1])
        } else {
            Some(&codepoints[0..2])
        }
    }
}

pub fn entity_lookup(name: &str) -> Option<String> {
    let name_size = name.len();
    let name = name.as_ptr() as *const c_char;

    let entity;

    let mut res = String::new();

    unsafe {
        #[cfg(target_pointer_width = "64")]
        {
            entity = sys::entity_lookup(name, name_size as u64);
        }
        #[cfg(target_pointer_width = "32")]
        {
            entity = sys::entity_lookup(name, name_size as u32);
        }

        if entity.is_null() {
            return None;
        }

        let c1 = (*entity).codepoints[0];
        let c2 = (*entity).codepoints[1];

        let c1 = char::from_u32_unchecked(c1);

        if c2 != 0 {
            let c2 = char::from_u32_unchecked(c2);
            res.extend([c1, c2].into_iter());
        } else {
            res.extend([c1].into_iter())
        }
    }
    Some(res)
}

#[cfg(test)]
mod test {
    use super::entity_lookup;
    use super::entity_lookup_raw;
    #[test]
    fn test1() {
        assert_eq!(entity_lookup("&lt;"), Some("<".to_owned()));
        assert_eq!(entity_lookup("&napE;"), Some("⩰̸".to_owned()));
        assert_eq!(entity_lookup("&noSuchEntity;"), None);
    }

    #[test]
    fn test2() {
        for i in 1..=500000 {
            assert_eq!(entity_lookup("&napE;"), Some("⩰̸".to_owned()));
        }
    }

    #[test]
    fn test3() {
        assert_eq!(
            entity_lookup_raw(b"&lt;"),
            Some(&[b'<' as u32] as &[u32])
        );
        assert_eq!(
            entity_lookup_raw(b"&napE;"),
            Some(&[10864u32, 824u32] as &[u32])
        );
        assert_eq!(entity_lookup_raw(b"&noSuchEntity;"), None);
    }
}
