use std::net::TcpStream;
use std::io::Write;
use ::ReadlineForTcpStream;
use ::page::Page;

pub fn main_page (stream: &mut TcpStream)
{
    stream.write(b"Hello pidr!\n\
                   Wellcome to SHAWERMA\n\
                   Our SHAWERMA best in the world (otvechau)\n\n\
                   Our BEST IN THE WORLD menu:\n\
                   1. Price list\n\
                   2. Buy\n\
                   3. Check your luck\n\
                   4. Exit\n").unwrap();

    loop {
        stream.write(b"\n> ").unwrap();
        let mut buffer = String::new();
        let len = stream.read_line(&mut buffer).unwrap();
        if len != 1 {continue}
        let c = buffer.chars().next().unwrap();

        match c {
            '1' => {
                let price_list = &::pages::PriceList;
                let price_list = Page::new(price_list);
                price_list.load_for(stream);
            },
            '2' => {stream.write(b"You'r selected \"Buy\", but IDITE HAHUI\n").unwrap();},
            '3' => {stream.write(b"You'r selected \"Check your luck\", but IDITE HAHUI\n").unwrap();},
            '4' => {
                stream.write(b"You'r selected \"Exit\", then IDITE HAHUI\n").unwrap();
                panic!("Kakoito pidor vyshel"); // he he he, bydlo-style mod true
            },
            _ => {stream.write(b"You'r selected smth shit, but IDITE HAHUI\n").unwrap();}
        };
    };

}
