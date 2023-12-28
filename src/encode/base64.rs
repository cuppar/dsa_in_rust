pub struct Base64Encoder;

impl Base64Encoder {
    pub fn encode(src: &[u8]) -> String {
        let n = src.len();
        let tail = n % 3;
        let group = n / 3;

        let mut result = vec![];

        for i in 0..group {
            let (u1, u2, u3) = (src[i * 3], src[i * 3 + 1], src[i * 3 + 2]);
            let t1 = u1 >> 2;
            result.push(t1);

            let mut t2 = u1 & 0x03;
            t2 <<= 4;
            t2 |= u2 >> 4;
            result.push(t2);

            let mut t3 = u2 & 0x0f;
            t3 <<= 2;
            t3 |= u3 >> 6;
            result.push(t3);

            let t4 = u3 & 0x3f;
            result.push(t4);
        }

        if tail == 1 {
            let u1 = src[n - 1];

            let t1 = u1 >> 2;
            result.push(t1);

            let mut t2 = u1 & 0x03;
            t2 <<= 4;
            result.push(t2);
            result.push(0x40);
            result.push(0x40);
        } else if tail == 2 {
            let u1 = src[n - 2];
            let u2 = src[n - 1];

            let t1 = u1 >> 2;
            result.push(t1);

            let mut t2 = u1 & 0x03;
            t2 <<= 4;
            t2 |= u2 >> 4;
            result.push(t2);

            let mut t3 = u2 & 0x0f;
            t3 <<= 2;
            result.push(t3);

            result.push(0x40);
        }

        for byte in &mut result {
            if *byte < 26 {
                *byte = *byte + b'A';
            } else if *byte < 52 {
                *byte = *byte - 26 + b'a';
            } else if *byte < 62 {
                *byte = *byte - 52 + b'0';
            } else if *byte == 62 {
                *byte = b'+';
            } else if *byte == 63 {
                *byte = b'/';
            } else if *byte == 64 {
                *byte = b'=';
            }
        }

        String::from_utf8_lossy(result.as_slice()).to_string()
    }

    pub fn decode(src: &str) -> Vec<u8> {
        let mut tail = 0;

        let mut src_nums = vec![];
        for byte in src.bytes() {
            if byte >= b'A' && byte <= b'Z' {
                src_nums.push(byte - b'A');
            } else if byte >= b'a' && byte <= b'z' {
                src_nums.push(byte - b'a' + 26);
            } else if byte >= b'0' && byte <= b'9' {
                src_nums.push(byte - b'0' + 52);
            } else if byte == b'+' {
                src_nums.push(62);
            } else if byte == b'/' {
                src_nums.push(63);
            } else if byte == b'=' {
                tail += 1;
            }
        }

        let group = src_nums.len() / 4;
        let mut result = vec![];

        for i in 0..group {
            let (t1, t2, t3, t4) = (
                src_nums[i * 4],
                src_nums[i * 4 + 1],
                src_nums[i * 4 + 2],
                src_nums[i * 4 + 3],
            );

            let mut u1 = t1 << 2;
            u1 |= t2 >> 4;
            result.push(u1);

            let mut u2 = (t2 & 0x0f) << 4;
            u2 |= t3 >> 2;
            result.push(u2);

            let mut u3 = (t3 & 0x03) << 6;
            u3 |= t4;
            result.push(u3);
        }

        if tail == 1 {
            let n = src_nums.len();
            let (t1, t2, t3) = (src_nums[n - 3], src_nums[n - 2], src_nums[n - 1]);

            let mut u1 = t1 << 2;
            u1 |= t2 >> 4;
            result.push(u1);

            let mut u2 = (t2 & 0x0f) << 4;
            u2 |= t3 >> 2;
            result.push(u2);
        } else if tail == 2 {
            let n = src_nums.len();
            let (t1, t2) = (src_nums[n - 2], src_nums[n - 1]);

            let mut u1 = t1 << 2;
            u1 |= t2 >> 4;
            result.push(u1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let source = "Hello Base64!".to_string();
        let target = "SGVsbG8gQmFzZTY0IQ==".to_string();

        assert_eq!(Base64Encoder::encode(source.as_bytes()), target);
        assert_eq!(Base64Encoder::decode(&target), source.as_bytes());
    }
}
