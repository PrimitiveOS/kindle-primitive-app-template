#[derive(Default)]
pub struct Hello {
    count: u32,
}

impl Hello {
    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn greet(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    pub fn restore(saved: &[u8]) -> Self {
        let count = std::str::from_utf8(saved).ok().and_then(|count| count.trim().parse().ok()).unwrap_or_default();
        Self { count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_count_is_restored() {
        let mut hello = Hello::restore(b"2\n");
        hello.greet();
        assert_eq!(hello.count(), 3);
        assert_eq!(Hello::restore(b"").count(), 0);
    }
}
