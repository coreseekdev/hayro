fn main() {
    let data = bytes_from_hex("000000003000000000000000000000260000000000000000000100000002000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000a");
    let settings = hayro_jbig2::DecodeSettings::default();
    dump_segments(&data);
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
    // torcular passes the PDF dict dims (1x3) the way pdfium's embedded
    // context does.
    let page = hayro_jbig2::Page::new_embedded_with_size(
        &data, None, &settings, Some(1), Some(3),
    )
    .unwrap();
    println!("page {}x{}", page.width(), page.height());
    let mut decoder = Gray { data: vec![255; page.width() as usize * page.height() as usize], position: 0 };
    match page.decode(&mut decoder) {
        Ok(()) => println!("pixels: {:?}", decoder.data),
        Err(e) => {
            // The release profile needs the explicit panic hook for the backtrace.
            std::panic::set_hook(Box::new(|info| {
                eprintln!("{info}");
                let bt = std::backtrace::Backtrace::force_capture();
                eprintln!("{bt}");
            }));
            panic!("decode failed after {} pixels: {e:?}", decoder.position)
        }
    }
    println!("pixels: {:?}", decoder.data);
}
fn dump_segments(data: &[u8]) {
    let mut i = 0usize;
    while i + 11 <= data.len() {
        let seg_no = u32::from_be_bytes(data[i..i+4].try_into().unwrap());
        let flags = data[i+4];
        let stype = flags & 0x3F;
        let rt = data[i+5];
        let count = (rt >> 5) & 7;
        let mut pos = if count == 7 { i + 9 } else { i + 6 + count as usize };
        let page = data[pos];
        pos += 1;
        let len = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;
        let avail = data.len() - pos;
        println!("@{i}: seg={seg_no} type={stype} page={page} len={len} avail={avail}");
        println!("   data head: {}", data[pos..(pos+24).min(data.len())].hex());
        if len == 0 || len > avail { break; }
        i = pos + len;
    }
}

trait Hex { fn hex(&self) -> String; }
impl Hex for [u8] {
    fn hex(&self) -> String { self.iter().map(|b| format!("{b:02x}")).collect() }
}

fn bytes_from_hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i+2], 16).unwrap()).collect()
}
