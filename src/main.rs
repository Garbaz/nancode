fn encode(bytes: &[u8]) -> Vec<f64> {
    bytes
        .chunks(6)
        .map(|chunk| {
            let mut encoded = f64::INFINITY.to_bits();

            for (i, c) in chunk.iter().enumerate() {
                encoded |= (*c as u64) << (i * 8);
            }
            f64::from_bits(encoded)
        })
        .collect()
}

fn decode(code: &[f64]) -> Vec<u8> {
    code.iter()
        .flat_map(|c| {
            let c = c.to_bits();
            (0..6).map(move |i| (c >> (i * 8)) as u8)
        })
        .collect()
}

fn main() {
    let message = "This is a secret message hidden in f64 NaN values.";

    println!("Original message:");
    println!("  {}", message);
    let code = encode(message.as_bytes());
    println!();

    println!("Encoded message:");
    println!("  {:?}", code);
    println!();

    println!("Encoded message `is_nan`:");
    println!(
        "  {:?}",
        code.iter().map(|c| c.is_nan()).collect::<Vec<_>>()
    );
    println!();

    println!("Encoded message as bits:");
    for c in &code {
        println!("  {:064b}", c.to_bits());
    }
    println!();

    println!("Decoded message:");
    let mut decoded = decode(&code[..]);
    decoded.retain(|c| *c != 0);
    println!("  {}", core::str::from_utf8(&decoded).unwrap());
}
