use std::io::stdin;

pub fn read_enty() -> String {
    let mut message = String::new();
    let stdin_reader = stdin();
    let reader_res = stdin_reader.read_line(&mut message);

    if reader_res.is_err() {
        println!("error! {:?}", reader_res.err());
    }

    message.trim().to_string()
}
