use aes::Aes256;
use blake3;
use fpe::ff1::{BinaryNumeralString, FF1};
use genTests::{k12, sp800_185, utils};
use serde::Serialize;
use serde_with::{serde_as, skip_serializing_none};

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize)]
struct TestCase {
    fn_name: String,
    #[serde_as(as = "serde_with::hex::Hex")]
    data: Vec<u8>,
    #[serde_as(as = "serde_with::hex::Hex")]
    exp: Vec<u8>,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    nist_fn: Option<Vec<u8>>,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    personalization: Option<Vec<u8>>,
    #[serde_as(as = "Option<serde_with::hex::Hex>")]
    key: Option<Vec<u8>>,
    block_len: Option<u64>,
}

#[derive(Serialize)]
struct TestCases {
    v: Vec<TestCase>,
}

impl TestCases {
    fn new() -> Self { TestCases { v: vec![] } }

    fn cshake128(&mut self, data: &[u8], dk_len: usize, name: &[u8], cust: &[u8]) {
        let exp = sp800_185::cshake128(data, dk_len, Some(name), Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "cshake128".to_string(),
            data: data.to_vec(),
            nist_fn: Some(name.to_vec()),
            personalization: Some(cust.to_vec()),
            key: None,
            block_len: None,
            exp,
        })
    }

    fn cshake256(&mut self, data: &[u8], dk_len: usize, name: &[u8], cust: &[u8]) {
        let exp = sp800_185::cshake256(data, dk_len, Some(name), Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "cshake256".to_string(),
            data: data.to_vec(),
            nist_fn: Some(name.to_vec()),
            personalization: Some(cust.to_vec()),
            key: None,
            block_len: None,
            exp,
        })
    }

    fn kmac128(&mut self, key: &[u8], data: &[u8], dk_len: usize, cust: &[u8]) {
        let exp = sp800_185::kmac128(key, data, dk_len, Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "kmac128".to_string(),
            data: data.to_vec(),
            key: Some(key.to_vec()),
            nist_fn: None,
            personalization: Some(cust.to_vec()),
            block_len: None,
            exp,
        })
    }

    fn kmac256(&mut self, key: &[u8], data: &[u8], dk_len: usize, cust: &[u8]) {
        let exp = sp800_185::kmac256(key, data, dk_len, Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "kmac256".to_string(),
            data: data.to_vec(),
            key: Some(key.to_vec()),
            nist_fn: None,
            personalization: Some(cust.to_vec()),
            block_len: None,
            exp,
        })
    }

    fn k12(&mut self, data: &[u8], dk_len: usize, cust: &[u8]) {
        let exp = k12::k12(data, dk_len, Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "k12".to_string(),
            data: data.to_vec(),
            key: None,
            nist_fn: None,
            personalization: Some(cust.to_vec()),
            block_len: None,
            exp,
        })
    }

    fn b3(&mut self, data: &[u8], dk_len: usize) {
        let mut h = blake3::Hasher::new();
        h.update(data);
        let mut exp = vec![0u8; dk_len];
        let mut output_reader = h.finalize_xof();
        output_reader.fill(&mut exp);
        self.v.push(TestCase {
            fn_name: "blake3".to_string(),
            data: data.to_vec(),
            key: None,
            nist_fn: None,
            personalization: None,
            block_len: None,
            exp,
        })
    }

    fn binary_ff1(&mut self, key: &[u8], data: &[u8]) {
        let ff = FF1::<Aes256>::new(key, 2).unwrap();
        let enc = ff.encrypt(&[], &BinaryNumeralString::from_bytes_le(data)).unwrap();
        self.v.push(TestCase {
            fn_name: "binary_ff1".to_string(),
            data: data.to_vec(),
            key: Some(key.to_vec()),
            nist_fn: None,
            personalization: None,
            block_len: None,
            exp: enc.to_bytes_le().to_vec(),
        });
    }

    fn parallel128(&mut self, data: &[u8], block_len: u64, dk_len: usize, cust: &[u8]) {
        let exp = sp800_185::parallel_hash128(data, block_len, dk_len, Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "parallel128".to_string(),
            block_len: Some(block_len),
            data: data.to_vec(),
            key: None,
            nist_fn: None,
            personalization: Some(cust.to_vec()),
            exp,
        })
    }

    fn parallel256(&mut self, data: &[u8], block_len: u64, dk_len: usize, cust: &[u8]) {
        let exp = sp800_185::parallel_hash256(data, block_len, dk_len, Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "parallel256".to_string(),
            block_len: Some(block_len),
            data: data.to_vec(),
            key: None,
            nist_fn: None,
            personalization: Some(cust.to_vec()),
            exp,
        })
    }

    fn tuple128(&mut self, data: &[u8], dk_len: usize, cust: &[u8]) {
        let mut v = Vec::new();
        for i in 0..data.len() {
            v.push(&data[0..i]);
        }
        let exp = sp800_185::tuple_hash128(&v, dk_len, Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "tuple128".to_string(),
            block_len: None,
            data: data.to_vec(),
            key: None,
            nist_fn: None,
            personalization: Some(cust.to_vec()),
            exp,
        })
    }

    fn tuple256(&mut self, data: &[u8], dk_len: usize, cust: &[u8]) {
        let mut v = Vec::new();
        for i in 0..data.len() {
            v.push(&data[0..i]);
        }
        let exp = sp800_185::tuple_hash256(&v, dk_len, Some(cust)).to_vec();
        self.v.push(TestCase {
            fn_name: "tuple256".to_string(),
            block_len: None,
            data: data.to_vec(),
            key: None,
            nist_fn: None,
            personalization: Some(cust.to_vec()),
            exp,
        })
    }

    fn to_json(&self) -> String { serde_json::to_string(&self).unwrap() }
}

fn main() {
    let r1 = utils::random(1, 4096);
    let r2 = utils::random(2, 4096);
    let r3 = utils::random(3, 4096);

    let mut a = TestCases::new();
    // 4096 takes 368 mb :(
    for i in 0..512 {
        // CSHAKE128
        a.cshake128(&r1[0..i], 16, &[], &[]);
        if i >= 1 {
            a.cshake128(&[], i, &[], &[]);
        }
        a.cshake128(&[], 16, &r2[0..i], &[]);
        a.cshake128(&[], 16, &[], &r3[0..i]);
        // CSHAKE256
        a.cshake256(&r1[0..i], 16, &[], &[]);
        if i >= 1 {
            a.cshake256(&[], i, &[], &[]);
        }
        a.cshake256(&[], 16, &r2[0..i], &[]);
        a.cshake256(&[], 16, &[], &r3[0..i]);
        // // KMAC128
        a.kmac128(&r1[0..i], &[], 16, &[]);
        a.kmac128(&[], &r2[0..i], 16, &[]);
        if i >= 1 {
            a.kmac128(&[], &[], i, &[]);
        }
        a.kmac128(&[], &[], 16, &r3[0..i]);
        // KMAC256
        a.kmac256(&r1[0..i], &[], 16, &[]);
        a.kmac256(&[], &r2[0..i], 16, &[]);
        if i >= 1 {
            a.kmac256(&[], &[], i, &[]);
        }
        a.kmac256(&[], &[], 16, &r3[0..i]);
        // K12
        a.k12(&r1[0..i], 16, &[]);
        if i >= 1 {
            a.k12(&[], i, &[]);
        }
        a.k12(&[], 16, &r2[0..i]);
        // B3
        a.b3(&r1[0..i], 16);
        a.b3(&[], i);

        // Parallel (block_len should be power of two and greater than lane
        // size(8))

        for block in [8, 16, 32, 64, 128] {
            // 128
            a.parallel128(&r1[0..i], block, 16, &[]);
            a.parallel128(&[], block, 16, &r2[0..i]);
            a.parallel128(&[], block, 16, &[]);
            // 256
            a.parallel256(&r1[0..i], block, 16, &[]);
            a.parallel256(&[], block, 16, &r2[0..i]);
            a.parallel256(&[], block, 16, &[]);
        }
        if i >= 1 {
            a.parallel128(&[], 8, i, &[]);
            a.parallel256(&[], 8, i, &[]);
        }
        // Tuple128
        a.tuple128(&r1[0..i], 16, &[]);
        if i >= 1 {
            a.tuple128(&[], i, &[]);
        }
        a.tuple128(&[], 16, &r2[0..i]);
        a.tuple128(&[], 16, &[]);
        // tuple256
        a.tuple256(&r1[0..i], 16, &[]);
        if i >= 1 {
            a.tuple256(&[], i, &[]);
        }
        a.tuple256(&[], 16, &r2[0..i]);
        a.tuple256(&[], 16, &[]);
        // a.binary_ff1(&r1[0..32], &r2[0..i]);
    }
    println!("{}", a.to_json());

    // let buf_4gb = vec![0u8; 1 << 32];
    // println!(
    //     "CSHAKE128 4GB: {:?}",
    //     hex::encode(sp800_185::cshake128(&buf_4gb, 16, Some(&buf_4gb),
    // Some(&buf_4gb))) );
    // println!(
    //     "CSHAKE256 4GB: {:?}",
    //     hex::encode(sp800_185::cshake256(&buf_4gb, 16, Some(&buf_4gb),
    // Some(&buf_4gb))) );
    // println!(
    //     "KMAC128 4GB: {:?}",
    //     hex::encode(sp800_185::kmac128(&buf_4gb, &buf_4gb, 16,
    // Some(&buf_4gb))) );
    // println!(
    //     "KMAC256 4GB: {:?}",
    //     hex::encode(sp800_185::kmac256(&buf_4gb, &buf_4gb, 16,
    // Some(&buf_4gb))) );
    // println!("K12 4GB: {:?}", hex::encode(k12::k12(&buf_4gb, 16,
    // Some(&buf_4gb)))); println!("Blake3 4GB: {:?}",
    // blake3::hash(&buf_4gb)); 2635 fails with js-sha3 too
    // (bc58e9c8534e5fa7346f06e6ab25e2db), should be
    // '031801b0b50ebeef772fbe7a279bc144'
    // Buffer.from(new
    // Uint8Array(require('js-sha3').kmac256.create(Buffer.from([]), 16*8,
    // Buffer.from('
    // 084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aad'
    // , 'hex')).update(Buffer([])).arrayBuffer())).toString('hex')==='
    // 031801b0b50ebeef772fbe7a279bc144'
}
