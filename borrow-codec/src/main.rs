use bytes::{Buf, BufMut, BytesMut};
use futures::SinkExt;
use futures::stream::{SplitSink, StreamExt};
use tokio::fs::File;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::{Decoder, Encoder, Framed};

struct State {
}

enum Item {
    Empty,
    Text(String)
}

impl Encoder<&Item> for State {
    type Error = std::io::Error;

    fn encode(&mut self, item: &Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            Item::Empty =>
                buf.put_u16(0x0001),
            Item::Text(s) => {
                buf.put_u16(0x0002);
                buf.put_u16(s.len() as u16);
                buf.put(s.as_bytes());
            }
        }
        Ok(())
    }
}

impl Decoder for State {
    type Item = Item;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Item>, Self::Error> {
        match buf.get_u16() {
            0x001 =>
                Ok(Some(Item::Empty)),
            0x002 => {
                let len = buf.get_u16() as usize;
                let sb = buf.split_to(len);

                let text = String::from_utf8(sb.to_vec()).unwrap();

                Ok(Some(Item::Text(text)))
            },
            _other =>
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown type".to_string()))
        }
    }
}

async fn sender<'item>(sink: &mut SplitSink<Framed<File, State>, &'item Item>, item: &'item Item) -> Result<(), Box<dyn std::error::Error>> {
    sink.send(item).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("foo.txt").await?;
    let text = Item::Text("Hey man".into());

    let (mut sink, _stream) = Framed::new(file, State{}).split();

    sender(&mut sink, &text).await?;

    sink.send(&Item::Empty).await?;

    Ok(())
}
