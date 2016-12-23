use errors::*;
use rustc_serialize as S;
use regex as R;

pub struct Decoder<'a> {
    captures: R::Captures<'a>,
    stack: Vec<String>,
}

impl<'a> Decoder<'a> {
    pub fn new(captures: R::Captures<'a>) -> Decoder<'a> {
        Decoder {
            captures: captures,
            stack: vec![],
        }
    }
}

macro_rules! read_primitive {
    ($name:ident, $ty:ident) => {
        fn $name(&mut self) -> Result<$ty> {
            match self.stack.pop() {
                None => Err("missing value".into()),
                Some(value) => value.parse().chain_err(|| "failed to decode primitive")
            }
        }
    }
}

impl<'a> S::Decoder for Decoder<'a> {
    type Error = Error;

    fn read_nil(&mut self) -> Result<()> {
        unimplemented!();
    }

    read_primitive! { read_usize, usize }
    read_primitive! { read_u64, u64 }
    read_primitive! { read_u32, u32 }
    read_primitive! { read_u16, u16 }
    read_primitive! { read_u8, u8 }

    read_primitive! { read_isize, isize }
    read_primitive! { read_i64, i64 }
    read_primitive! { read_i32, i32 }
    read_primitive! { read_i16, i16 }
    read_primitive! { read_i8, i8 }

    read_primitive! { read_f64, f64 }
    read_primitive! { read_f32, f32 }

    fn read_bool(&mut self) -> Result<bool> {
        unimplemented!();
    }

    fn read_char(&mut self) -> Result<char> {
        match self.stack.pop() {
            None => Err("missing value".into()),
            Some(value) => {
                let mut chars = value.chars();

                let c = match chars.next() {
                    None => return Err("missing value".into()),
                    Some(c) => c,
                };

                match chars.next() {
                    None => Ok(c),
                    Some(_) => Err("extra characters found".into()),
                }
            }
        }
    }

    fn read_str(&mut self) -> Result<String> {
        match self.stack.pop() {
            None => Err("missing value".into()),
            Some(value) => Ok(value.into())
        }
    }

    fn read_enum<T, F>(&mut self, name: &str, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        unimplemented!();
    }

    fn read_enum_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T> where F: FnMut(&mut Self, usize) -> Result<T> {
        unimplemented!();
    }

    fn read_enum_variant_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        unimplemented!();
    }

    fn read_enum_struct_variant<T, F>(&mut self, names: &[&str], f: F) -> Result<T> where F: FnMut(&mut Self, usize) -> Result<T> {
        unimplemented!();
    }

    fn read_enum_struct_variant_field<T, F>(&mut self, f_name: &str, f_idx: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        unimplemented!();
    }

    fn read_struct<T, F>(&mut self, s_name: &str, _: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        f(self)
    }

    fn read_struct_field<T, F>(&mut self, f_name: &str, _: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        match self.captures.name(f_name) {
            None => Err("missing field name".into()),
            Some(val) => {
                self.stack.push(val.to_string());
                f(self)
            }
        }
    }

    fn read_tuple<T, F>(&mut self, _: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        f(self)
    }

    fn read_tuple_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        // a_idx + 1 because capture 0 is the whole match
        match self.captures.at(a_idx + 1) {
            None => Err("missing tuple arg".into()),
            Some(val) => {
                self.stack.push(val.to_string());
                f(self)
            }
        }
    }

    fn read_tuple_struct<T, F>(&mut self, s_name: &str, len: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        unimplemented!();
    }

    fn read_tuple_struct_arg<T, F>(&mut self, a_idx: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        unimplemented!();
    }

    fn read_option<T, F>(&mut self, mut f: F) -> Result<T> where F: FnMut(&mut Self, bool) -> Result<T> {
        let value = self.stack.pop();

        match value {
            None => f(self, false),
            Some(value) => {
                self.stack.push(value);
                f(self, true)
            }
        }
    }

    fn read_seq<T, F>(&mut self, f: F) -> Result<T> where F: FnOnce(&mut Self, usize) -> Result<T> {
        let mut temp = vec![];
        for val in self.captures.iter().skip(1) {
            temp.push(val.unwrap().to_string());
        }
        temp.reverse();
        let len = temp.len();
        self.stack.extend(temp);
        f(self, len)
    }

    fn read_seq_elt<T, F>(&mut self, _: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        f(self)
    }

    fn read_map<T, F>(&mut self, f: F) -> Result<T> where F: FnOnce(&mut Self, usize) -> Result<T> {
        unimplemented!();
    }

    fn read_map_elt_key<T, F>(&mut self, idx: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        unimplemented!();
    }

    fn read_map_elt_val<T, F>(&mut self, idx: usize, f: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T> {
        unimplemented!();
    }

    fn error(&mut self, err: &str) -> Self::Error {
        err.into()
    }
}

pub fn decode<T: S::Decodable>(regex: &R::Regex, string: &str) -> Result<T> {
    match regex.captures(string) {
        None => Err("regex failed to match against text".into()),
        Some(captures) => {
            let mut decoder = Decoder::new(captures);
            S::Decodable::decode(&mut decoder)
        }
    }
}

#[cfg(test)]
mod tests {
    use regex as R;
    use super::*;

    #[test]
    fn decode_struct_with_strings() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: String,
            pub year: String,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        let val = decode::<Capture>(&re, &text).unwrap();

        assert_eq!(&val.title, "Citizen Kane");
        assert_eq!(&val.year, "1941");
    }

    #[test]
    fn decode_struct_with_usize() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: String,
            pub year: usize,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        let val = decode::<Capture>(&re, &text).unwrap();

        assert_eq!(&val.title, "Citizen Kane");
        assert_eq!(val.year, 1941);
    }

    #[test]
    fn decode_struct_with_u64() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: String,
            pub year: u64,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        let val = decode::<Capture>(&re, &text).unwrap();

        assert_eq!(&val.title, "Citizen Kane");
        assert_eq!(val.year, 1941);
    }

    #[test]
    fn decode_struct_with_u32() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: String,
            pub year: u32,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        let val = decode::<Capture>(&re, &text).unwrap();

        assert_eq!(&val.title, "Citizen Kane");
        assert_eq!(val.year, 1941);
    }

    #[test]
    fn decode_struct_with_u16() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: String,
            pub year: u16,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        let val = decode::<Capture>(&re, &text).unwrap();

        assert_eq!(&val.title, "Citizen Kane");
        assert_eq!(val.year, 1941);
    }

    #[test]
    #[should_panic]
    fn decode_struct_with_u8_too_large() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: String,
            pub year: u8,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        // This will panic because 1941 won't fit into a u8
        decode::<Capture>(&re, &text).unwrap();
    }

    #[test]
    fn decode_struct_with_u8() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: String,
            pub year: u8,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{2})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (41).";

        let val = decode::<Capture>(&re, &text).unwrap();

        assert_eq!(&val.title, "Citizen Kane");
        assert_eq!(val.year, 41);
    }

    #[test]
    #[should_panic]
    fn decode_struct_with_char_too_large() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: char,
            pub year: usize,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        // This will panic because Citizen Kane won't fit into a char
        decode::<Capture>(&re, &text).unwrap();
    }

    #[test]
    fn decode_struct_with_char() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: char,
            pub year: usize,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)")
                           .unwrap();
        let text = "Not my favorite movie: 'C' (1941).";

        let val = decode::<Capture>(&re, &text).unwrap();

        assert_eq!(val.title, 'C');
        assert_eq!(val.year, 1941);
    }

    #[test]
    fn decode_struct_with_option() {
        #[derive(RustcDecodable)]
        struct Capture {
            pub title: String,
            pub year: Option<usize>,
        }

        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})?\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        let val = decode::<Capture>(&re, &text).unwrap();

        assert_eq!(val.title, "Citizen Kane");
        assert_eq!(val.year.is_some(), true);
        assert_eq!(val.year.unwrap(), 1941);
    }

    #[test]
    fn decode_tuple() {
        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})?\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        let (title, year) = decode::<(String, usize)>(&re, &text).unwrap();

        assert_eq!(title, "Citizen Kane");
        assert_eq!(year, 1941);
    }

    #[test]
    fn decode_vec_string() {
        let re = R::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})?\)")
                           .unwrap();
        let text = "Not my favorite movie: 'Citizen Kane' (1941).";

        let val = decode::<Vec<String>>(&re, &text).unwrap();

        assert_eq!(val[0], "Citizen Kane");
        assert_eq!(val[1], "1941");
    }
}
