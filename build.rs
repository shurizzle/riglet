fn main() {
    test::generate();
}

mod test {
    use std::{
        env,
        fs::{canonicalize, read_dir, DirEntry, File},
        io::Write,
        path::Path,
    };

    pub fn generate() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let destination = Path::new(&out_dir).join("tests.rs");
        let mut test_file = File::create(&destination).unwrap();

        let test_fonts = read_dir("./fonts/plain/").unwrap();

        for font in test_fonts {
            write_test("plain_", &mut test_file, &font.unwrap());
        }

        if cfg!(feature = "zip") {
            let test_fonts = read_dir("./fonts/zipped/").unwrap();

            for font in test_fonts {
                write_test("zipped_", &mut test_file, &font.unwrap());
            }
        }
    }

    fn write_test(prefix: &str, test_file: &mut File, entry: &DirEntry) {
        if entry.file_type().unwrap().is_file() && entry.path().extension().unwrap() == "flf" {
            let p = canonicalize(entry.path()).unwrap();
            let path = p.to_str().unwrap();
            let test_name = p.file_name().unwrap().to_str().unwrap();
            let test_name = &test_name[..(test_name.len() - 4)];

            write!(
                test_file,
                include_str!("./tests/test_template"),
                name = format!("{}{}", prefix, test_name),
                path = path
            )
            .unwrap();
        }
    }
}
