
#[derive(Debug, Clone)]
pub struct StrParser {
    s: Vec<char>,
    i: usize,
    skip_ws: bool,
}

impl StrParser {
    pub fn new(src: &str) -> StrParser {
        StrParser { s: src.chars().collect(), i: 0, skip_ws: false }
    }
    pub fn new_skip_ws(src: &str) -> StrParser {
        StrParser { s: src.chars().collect(), i: 0, skip_ws: true }
    }

    #[inline]
    pub fn full_string(&self) -> String {
        self.s.iter().collect::<String>()
    }
    #[inline]
    pub fn remaining_string(&self) -> String {
        self.s[self.i..].iter().collect::<String>()
    }

    #[inline]
    fn err(&self, msg: &str) -> String {
        if self.i < self.s.len() {
            format!("Parse error at column {}, {}, found {}", self.i, msg, self.s[self.i..].iter().collect::<String>())
        } else {
            format!("Parse error at end of line, {}", msg)
        }
    }

    #[inline]
    pub fn advance(&mut self, n: usize) { self.i = self.i.saturating_add(n).min(self.s.len()) }
    #[inline]
    pub fn retreat(&mut self, n: usize) { self.i = self.i.saturating_sub(n) }
    #[inline]
    pub fn has_more(&self) -> bool { self.i < self.s.len() }
    #[inline]
    pub fn is_finished(&self) -> bool { self.i >= self.s.len() }

    #[inline]
    pub fn skip_ws(&mut self) {
        while self.i < self.s.len() && self.s[self.i].is_ascii_whitespace() { self.i += 1; }
    }

    #[inline]
    pub fn skip_char(&mut self, ch: char) -> usize {
        let mut next = self.i;
        while next < self.s.len() && ch == self.s[next] {
            next += 1;
        }
        let rv = next - self.i;
        self.i = next;
        if self.skip_ws { self.skip_ws(); }
        return rv;
    }

    #[inline]
    pub fn next(&mut self) -> Option<char> {
        let rv;
        if self.i < self.s.len() { self.i += 1; rv = Some(self.s[self.i-1]); }
        else { rv = None; }
        if self.skip_ws { self.skip_ws(); }
        return rv;
    }

    #[inline]
    pub fn step(&mut self) -> Result<char, String> {
        let rv;
        if self.i < self.s.len() { self.i += 1; rv = Ok(self.s[self.i-1]); }
        else { rv = Err(self.err("unexpected end of string")); }
        if self.skip_ws { self.skip_ws(); }
        return rv;
    }

    #[inline]
    pub fn peek(&self) -> Option<char> {
        if self.i < self.s.len() { Some(self.s[self.i]) } else { None }
    }

    #[inline]
    pub fn is_numeric_next(&self) -> bool { self.i < self.s.len() && self.s[self.i].is_ascii_digit() }
    #[inline]
    pub fn is_alnum_next(&self) -> bool { self.i < self.s.len() && self.s[self.i].is_ascii_alphanumeric() }
    #[inline]
    pub fn is_char_next(&self, ch: char) -> bool { self.i < self.s.len() && ch == self.s[self.i] }

    #[inline]
    pub fn expect_char(&mut self, ch: char) -> Result<usize, String> {
        let mut next = self.i;
        while next < self.s.len() && ch == self.s[next] {
            next += 1;
        }
        if self.i == next { return Err(self.err(&format!("expected character '{}'", ch))); }
        let rv = Ok(next - self.i);
        self.i = next;
        if self.skip_ws { self.skip_ws(); }
        return rv;
    }

    #[inline]
    pub fn expect_eol(&mut self) -> Result<(), String> {
        if self.skip_ws { self.skip_ws(); }
        if self.i >= self.s.len() { Ok(()) } else { Err(self.err("expected end of string")) }
    }

    #[inline]
    pub fn extract_u64(&mut self) -> Result<u64, String> {
        let mut next = self.i;
        while next < self.s.len() && self.s[next].is_ascii_digit() {
            next += 1;
        }
        if self.i == next { return Err(self.err("expected an integer")); }
        let rv = Ok(self.s[self.i..next].iter().collect::<String>().parse::<u64>().map_err(|e| e.to_string())?);
        self.i = next;
        if self.skip_ws { self.skip_ws(); }
        return rv;
    }

    #[inline]
    pub fn extract_f64(&mut self) -> Result<f64, String> {
        let mut next = self.i;
        let mut has_decimal = false;
        while next < self.s.len() && (self.s[next].is_ascii_digit() || (!has_decimal && '.' == self.s[next])) {
            if '.' == self.s[next] { has_decimal = true; }
            next += 1;
        }
        if self.i == next { return Err(self.err("expected a number")); }
        let rv = Ok(self.s[self.i..next].iter().collect::<String>().parse::<f64>().map_err(|e| e.to_string())?);
        self.i = next;
        if self.skip_ws { self.skip_ws(); }
        return rv;
    }

    #[inline]
    pub fn extract_alnum(&mut self) -> Result<String, String> {
        let mut next = self.i;
        while next < self.s.len() && self.s[next].is_ascii_alphanumeric() {
            next += 1;
        }
        if self.i == next { return Err(self.err("expected alphanumeric")); }
        let rv = Ok(self.s[self.i..next].iter().collect::<String>());
        self.i = next;
        if self.skip_ws { self.skip_ws(); }
        return rv;
    }

    #[inline]
    pub fn extract_ident(&mut self) -> Result<String, String> {
        let mut next = self.i;
        while next < self.s.len() && (self.s[next].is_ascii_alphanumeric() || '_' == self.s[next]) {
            next += 1;
        }
        if self.i == next { return Err(self.err("expected an identifier")); }
        let rv = Ok(self.s[self.i..next].iter().collect::<String>());
        self.i = next;
        if self.skip_ws { self.skip_ws(); }
        return rv;
    }

}
