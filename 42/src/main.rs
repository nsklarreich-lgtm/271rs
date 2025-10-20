use std::convert::TryInto;

const K: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc, 0x3956c25bf348b538, 0x59f111f1b605d019,
    0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242,
    0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235,
    0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 0x2de92c6f592b0275,
    0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f,
    0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725,
    0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc,
    0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6,
    0x92722c851482353b, 0xa2bfe8a14cf10364, 0xa81a664bbc423001,
    0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218,
    0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc,
    0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915,
    0xc67178f2e372532b, 0xca273eceea26619c, 0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba,
    0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];

fn pad_message(message: &[u8]) -> Vec<u8> {
    let bit_len = (message.len() as u128) * 8;

    let mut padded = message.to_vec();
    padded.push(0x80);

    
    while (padded.len() % 128) != 112 {
        padded.push(0x00);
    }

    
    padded.extend_from_slice(&bit_len.to_be_bytes());

    assert_eq!(padded.len() % 128, 0);
    padded
}

fn create_blocks(padded: &[u8]) -> Vec<[u8; 128]> {
    assert_eq!(padded.len() % 128, 0);
    let mut blocks = Vec::with_capacity(padded.len() / 128);
    for chunk in padded.chunks_exact(128) {
        let mut b = [0u8; 128];
        b.copy_from_slice(chunk);
        blocks.push(b);
    }
    blocks
}

fn block_to_word(block: &[u8; 128]) -> [u64; 16] {
    let mut w = [0u64; 16];
    for i in 0..16 {
        let start = i * 8;
        let bytes: [u8; 8] = block[start..start + 8].try_into().unwrap();
        w[i] = u64::from_be_bytes(bytes);
    }
    w
}

fn message_schedule(words16: [u64; 16]) -> [u64; 80] {
    let mut w = [0u64; 80];
    w[..16].copy_from_slice(&words16);

    for t in 16..80 {
        let s0 = w[t - 15].rotate_right(1) ^ w[t - 15].rotate_right(8) ^ (w[t - 15] >> 7);
        let s1 = w[t - 2].rotate_right(19) ^ w[t - 2].rotate_right(61) ^ (w[t - 2] >> 6);
        w[t] = w[t - 16]
            .wrapping_add(s0)
            .wrapping_add(w[t - 7])
            .wrapping_add(s1);
    }

    w
}

fn compress(state: &mut [u64; 8], w: &[u64; 80]) {
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    for t in 0..80 {
        let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[t])
            .wrapping_add(w[t]);

        let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(maj);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

pub fn sha512(message: &[u8]) -> [u8; 64] {
    
    let mut state: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];

    
    let padded =pad_message(message);
    let blocks = create_blocks(&padded);

    for block in blocks.iter() {
        let words16 = block_to_word(block);
        let w = message_schedule(words16);
        compress(&mut state, &w);
    }

    
    let mut out = [0u8; 64];
    for (i, word) in state.iter().enumerate() {
        let bytes = word.to_be_bytes();
        out[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
    }

    out
}


use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() == 1 {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        print_hash(&input, "-");
        return;
    }

    
    for filename in &args[1..] {
        match fs::read(filename) {
            Ok(data) => print_hash(&data, filename),
            Err(e) => eprintln!("sha512: {}: {}", filename, e),
        }
    }
}

fn print_hash(data: &[u8], label: &str) {
    let hash = sha512(data);
    for b in &hash {
        print!("{:02x}", b);
    }
    println!("  {}", label);
}

    



