use sha3::{Digest, Keccak256};
use tiny_keccak::{Hasher, Keccak};

fn main() {
    // Define your input bits as a Vec<u8> with values 0 and 1
    let input_bits: Vec<bool> = vec![
        false, true, true, true, false, true, false, false, false, true, true, false, false, true,
        false, true, false, true, true, true, false, false, true, true, false, true, true, true,
        false, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false,
    ];
    let input_bytes: Vec<u8> = bits_to_bytes(&input_bits);
    println!("Input bits: {:?}", &input_bits);
    println!("Input bytes: {:?}", &input_bytes);

    // Calculate the Keccak-256 hash
    let mut hasher = Keccak::v256();
    hasher.update(&input_bytes);
    let mut output_bytes = [0u8; 32];
    hasher.finalize(&mut output_bytes);
    // Convert the output bytes into a Vec<u8> representing bits
    let output_bits: Vec<u8> = bytes_to_bits(&output_bytes);

    // Print the input and output bits
    println!("Output bits tiny: {:?}", &output_bits);
    println!("Output bytes tiny: {:?}", &output_bytes);

    // From https://github.com/tchataigner/to_delete/blob/main/test/keccak.js#L21-L41
    let to_delete_js_output = vec![
        true, false, true, false, false, true, false, false, true, false, false, false, true,
        false, false, false, false, true, false, false, false, true, true, false, true, true, true,
        false, false, false, false, true, true, false, false, false, false, true, false, true,
        false, true, false, false, true, true, false, true, false, false, false, true, true, false,
        true, false, true, false, false, false, false, true, true, false, true, false, true, true,
        true, true, true, false, false, true, true, false, true, false, false, true, true, true,
        true, true, false, false, false, true, true, false, false, false, false, false, true,
        false, false, false, true, false, false, true, true, true, true, true, false, false, true,
        false, true, true, false, true, false, true, false, true, false, true, true, false, true,
        false, false, false, false, true, true, false, false, true, true, false, false, true, true,
        false, false, true, false, false, false, false, false, false, false, true, true, false,
        true, false, false, false, true, false, true, false, true, true, false, false, true, false,
        false, false, false, false, false, false, true, false, true, false, true, true, true, true,
        true, true, false, true, false, true, false, true, true, true, false, true, true, true,
        true, false, false, true, true, true, true, true, false, true, false, false, true, false,
        true, true, false, true, true, false, true, false, true, true, false, false, true, false,
        true, true, true, false, false, true, false, true, false, true, false, true, true, false,
        true, false, true, false, true, false, true, false, true, true, false, true, false, false,
        true, false, true, false, true, false, false, false,
    ];
    // See https://github.com/tchataigner/circom-scotia/blob/feature/keccak-example/examples/keccak.rs
    let circom_scotia_output = vec![
        true, false, true, true, false, true, true, false, false, true, false, true, false, false,
        true, false, false, false, false, true, false, true, true, false, false, false, true,
        false, false, true, false, true, false, true, false, true, false, false, true, true, true,
        false, false, false, false, true, false, false, true, true, false, true, false, true,
        false, false, true, false, true, false, false, false, true, false, false, true, false,
        false, false, true, false, false, true, false, false, false, true, false, true, false,
        true, false, false, true, false, false, false, false, false, true, false, false, true,
        false, true, true, false, false, false, true, false, false, false, false, true, true,
        false, false, false, false, false, true, false, false, false, true, false, true, false,
        true, false, true, false, false, true, true, true, true, false, true, false, false, true,
        true, false, false, false, true, true, false, true, false, false, true, true, true, false,
        true, true, true, false, true, true, true, false, false, true, false, false, false, true,
        true, true, true, false, false, true, false, true, false, true, false, false, false, false,
        false, true, true, false, true, true, true, true, false, true, true, true, false, false,
        true, false, true, true, false, true, false, true, true, true, false, false, false, false,
        false, true, false, false, false, true, true, false, false, false, false, false, false,
        false, false, false, true, true, false, false, false, true, true, true, false, true, true,
        true, true, false, true, true, false, false, true, true, false, true, true, false, true,
        false, false, true, false, true, true, false, false, false, false, false, true,
    ];
    println!(
        "Output bits to_delete js: {:?}",
        &to_delete_js_output
            .iter()
            .map(|b| {
                if *b {
                    1
                } else {
                    0
                }
            })
            .collect::<Vec<u8>>()
    );
    println!(
        "Output bytes to_delete js: {:?}",
        js_bits_to_bytes(&to_delete_js_output)
    );
    println!(
        "Output bits circom-scotia: {:?}",
        &circom_scotia_output
            .iter()
            .map(|b| {
                if *b {
                    1
                } else {
                    0
                }
            })
            .collect::<Vec<u8>>()
    );
    // From https://zkrepl.dev/?gist=da2089a9df36f54758dc496a208d489f
    let zkrepl_output = vec![
        true, false, true, true, false, true, true, false, false, true, false, true, false, false,
        true, false, false, false, false, true, false, true, true, false, false, false, true,
        false, false, true, false, true, false, true, false, true, false, false, true, true, true,
        false, false, false, false, true, false, false, true, true, false, true, false, true,
        false, false, true, false, true, false, false, false, true, false, false, true, false,
        false, false, true, false, false, true, false, false, false, true, false, true, false,
        true, false, false, true, false, false, false, false, false, true, false, false, true,
        false, true, true, false, false, false, true, false, false, false, false, true, true,
        false, false, false, false, false, true, false, false, false, true, false, true, false,
        true, false, true, false, false, true, true, true, true, false, true, false, false, true,
        true, false, false, false, true, true, false, true, false, false, true, true, true, false,
        true, true, true, false, true, true, true, false, false, true, false, false, false, true,
        true, true, true, false, false, true, false, true, false, true, false, false, false, false,
        false, true, true, false, true, true, true, true, false, true, true, true, false, false,
        true, false, true, true, false, true, false, true, true, true, false, false, false, false,
        false, true, false, false, false, true, true, false, false, false, false, false, false,
        false, false, false, true, true, false, false, false, true, true, true, false, true, true,
        true, true, false, true, true, false, false, true, true, false, true, true, false, true,
        false, false, true, false, true, true, false, false, false, false, false, true,
    ];
    println!(
        "Output bits zkrepl: {:?}",
        &zkrepl_output
            .iter()
            .map(|b| {
                if *b {
                    1
                } else {
                    0
                }
            })
            .collect::<Vec<u8>>()
    );
}

fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut bits = Vec::new();
    for &byte in bytes {
        for i in (0..8).rev() {
            bits.push((byte >> i) & 1);
        }
    }
    bits
}

// Rust bit to byte implementation
// // Fills the byte from most important (leftmost) to the least significant (rightmost) bit
fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    let mut bytes = Vec::new(); // Create a new, empty vector to store bytes

    for chunk in bits.chunks(8) {
        // Iterate over the bits in chunks of 8
        let mut byte = 0u8; // Initialize a new byte to 0
        for (i, &bit) in chunk.iter().enumerate() {
            // Iterate over each bit in the chunk
            if bit {
                // If the current bit is true,
                byte |= 1 << (7 - i); // Set the corresponding bit in the byte
            }
        }
        bytes.push(byte); // Add the composed byte to the vector
    }
    bytes // Return the vector of bytes
}

// Implemented as https://github.com/vocdoni/keccak256-circom/blob/master/test/utils.js#L76-L89
// Fills the byte from least to most significant bit
fn js_bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    let mut bytes = vec![0; (bits.len() + 7) / 8]; // Initialize a vector with zeroes

    for (i, &bit) in bits.iter().enumerate() {
        // Iterate over each bit with its index
        let byte_index = i / 8; // Calculate the byte index for the current bit
        if bit {
            // If the current bit is true,
            bytes[byte_index] |= 1 << (i % 8); // Set the corresponding bit in the byte
        }
    }
    bytes // Return the array of bytes
}
