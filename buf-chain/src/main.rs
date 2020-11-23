use bytes::{Buf, BufMut, Bytes, BytesMut};

fn chain_with_slices() {
    let mut main = BytesMut::with_capacity(256);

    let params = Bytes::from(&b"parameters"[..]);

    let len = params.len() as u32;
    let length = [(len >> 24) as u8, (len >> 16) as u8, (len >> 8) as u8, len as u8];
    let total = (&b"\x00HEAD"[..]).chain(&length[..]).chain(params);

    main.put(total);

    dump(&mut main);
}

fn chain_with_bufs() {
    let first = Bytes::from(&b"Hey, "[..]);
    let second = Bytes::from(&b"world"[..]);

    let mut chained = first.chain(second);

    dump(&mut chained);
}

fn chain_with_mut_bufs() {
    let mut first = BytesMut::with_capacity(128);
    first.put_u16(0x1122);
    first.put(&b"Hey man! "[..]);

    let mut second = BytesMut::with_capacity(128);
    second.put_u32(0xAABBCCDD);

    let mut chained = first.chain(second);

    dump(&mut chained);
}

fn main() {
    chain_with_slices();
    chain_with_bufs();
    chain_with_mut_bufs();
}

fn dump(buf: &mut dyn Buf) {
    let mut i: usize = 0;
    let mut text: Vec<u8> = Vec::new();

    println!("---");

    while buf.has_remaining() {
        let b = buf.get_u8();

        print!("{:02X} ", b);

        if (b as char).is_alphanumeric() {
            text.push(b);
        } else {
            text.push(b'.');
        }

        i += 1;

        if i % 16 == 0 {
            println!("{}", std::str::from_utf8(&text).unwrap_or_default());
            text.clear();
        }
    }

    println!("---");
}
