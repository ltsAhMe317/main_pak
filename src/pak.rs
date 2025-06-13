use main_pak;

fn main() {
    let mut args = std::env::args();
    args.next();
    main_pak::pak_path(args.next().unwrap()).save("main.pak");
}
