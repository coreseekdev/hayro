fn main() {
    let data = bytes_from_hex("000000003000000000000000000000260000000000000000000100000002000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000a");
    let settings = hayro_jbig2::DecodeSettings::default();
    struct Gray {
        data: Vec<u8>,
        position: usize,
    }
    impl hayro_jbig2::Decoder for Gray {
        fn push_pixel(&mut self, black: bool) {
            if let Some(slot) = self.data.get_mut(self.position) {
                *slot = if black { 0 } else { 255 };
            }
            self.position += 1;
        }
        fn push_pixel_chunk(&mut self, black: bool, chunk_count: u32) {
            let count = chunk_count as usize * 8;
            let start = self.position.min(self.data.len());
            let end = self.position.saturating_add(count).min(self.data.len());
            self.data[start..end].fill(if black { 0 } else { 255 });
            self.position += count;
        }
        fn next_line(&mut self) {}
    }
    let page = hayro_jbig2::Page::new_embedded(&data, None, &settings).unwrap();
    println!("page {}x{}", page.width(), page.height());
    let mut decoder = Gray { data: vec![255; page.width() as usize * page.height() as usize], position: 0 };
    match page.decode(&mut decoder) {
        Ok(()) => println!("pixels: {:?}", decoder.data),
        Err(e) => eprintln!("decode failed after {} pixels: {e:?}", decoder.position),
    }
    println!("pixels: {:?}", decoder.data);
}
fn bytes_from_hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i+2], 16).unwrap()).collect()
}
