use molpe::pe;
use molpe::exec;

fn main() {
    let file = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("molpe <exe> <-- provide an exe to run");
        std::process::exit(1);
    });

    let fi = std::fs::File::open(file).unwrap();

    let mut reader = pe::Reader::new(fi);
    let mut image = reader.create_image().unwrap();
    let method = reader.read_entry_method(&mut image).unwrap();

    exec::Interpreter::default().run(&mut image, method);
}
