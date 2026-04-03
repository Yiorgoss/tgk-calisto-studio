// mod alter_uid;
mod add_global_styles;
mod convert_to_bold;
mod custom_font;
use glob::glob;
use std::{env, error::Error, fs::{self, File}, io::Write, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("starting...");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 1 {
        for entry in glob(&format!("./test_ebooks/{}/**/*.xhtml", args[1])[..])
            .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => overwrite_file(path.as_path())?,
                Err(e) => panic!("{}", e),
            }
        }
    }
    Ok(())
}
fn overwrite_file(path: &Path) -> Result<(), Box<dyn Error>>  {
    println!("Overwriting {:?}", path);
    let bytes = fs::read(path)?;
    // let converted = convert_to_bold::insert_bold_tags(bytes.as_slice(), true);
    let mut file = File::create(path)?; 
    file.write(&bytes)?;
    // file.write(&converted)?;
 
    Ok(())
}
// let input: &str = "";
// let input: &str = "<p class=\"calibre10\">“No signal. No Don Salvara. Can you breathe?”</p>";

// let converted = convert_to_bold::insert_bold_tags(input.as_bytes(), true);
// // let converted = add_global_styles::add_css(
// //     input.as_bytes(),
// //     true,
// //     true,
// //     400,
// //     500,
// //     String::from("asrt"),
// //     String::from("fonxxxt_name"),
// // );
// // let converted = custom_font::insert_custom_font("filename");
// // let converted = custom_font::insert_into_content_opf(input.as_bytes(), "xxxx");
// println!("{}", input);
// println!("{}", str::from_utf8(&converted.to_owned()).unwrap())
// }
