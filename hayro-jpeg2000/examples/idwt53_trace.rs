// Dumps the 5/3 reconstruction of known signals through the crate's own
// lifting machinery, for the cas=0/cas=1 comparison against pdfium's
// patched dwt.c.  The interleave output row = the final samples.
fn main() {
    let signals: [(bool, Vec<f32>, Vec<f32>); 4] = [
        // (low_first, low, high)
        (true, vec![10.0, 20.0, 30.0, 40.0], vec![1.0, 2.0, 3.0, 4.0]),
        (false, vec![10.0, 20.0, 30.0, 40.0], vec![1.0, 2.0, 3.0, 4.0]),
        (true, vec![10.0, 20.0, 30.0], vec![1.0, 2.0, 3.0]),
        (false, vec![10.0, 20.0, 30.0], vec![1.0, 2.0, 3.0]),
    ];
    for (i, (low_first, low, high)) in signals.into_iter().enumerate() {
        let out = hayro_jpeg2000::trace_53_for_example(low_first, &low, &high);
        println!(
            "case {i}: low_first={low_first} low={low:?} high={high:?} -> {out:?}"
        );
        // The same bands through the openjpeg-semantics reference: cas=0
        // rows are [low][high], cas=1 rows are [high][low].
        let sn = low.len();
        let mut row = Vec::with_capacity(sn + high.len());
        if low_first {
            row.extend_from_slice(&low);
            row.extend_from_slice(&high);
        } else {
            row.extend_from_slice(&high);
            row.extend_from_slice(&low);
        }
        let oj = hayro_jpeg2000::openjpeg_idwt53_h_for_example(&row, sn, u32::from(!low_first));
        println!("case {i}: openjpeg -> {oj:?}");
    }
}
