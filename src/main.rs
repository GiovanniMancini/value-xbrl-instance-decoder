use futures::stream::TryStreamExt; // for map_err
use quick_xml::events::Event;
use quick_xml::Reader;
use std::error::Error;
use tokio_util::io::StreamReader;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let x =
        "https://www.sec.gov/Archives/edgar/data/320193/000032019322000108/aapl-20220924_htm.xml";
    get_xml_stream(x).await;
}

fn convert_err(_err: reqwest::Error) -> std::io::Error {
    todo!()
}

async fn get_xml_stream(source: &str) -> Result<(), Box<dyn Error>> {
    let stream = reqwest::get(source).await?.bytes_stream();
    let mut reader = Reader::from_reader(StreamReader::new(stream.map_err(convert_err)));

    reader.trim_text(true);

    let mut count = 0;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into_async(&mut buf).await {
            Ok(Event::Start(_)) => count += 1,
            Ok(Event::Text(e)) => println!("Count {:?}. \n Text{:?}\n\n\n", count, e.unescape().unwrap().into_owned()),
            Err(e) => println!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }
        
        buf.clear();
    }

    Ok(())
}
