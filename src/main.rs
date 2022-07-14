use argh::FromArgs;
use elf_rs::{self, Elf, ElfClass, ElfFile};
use iced_x86::{CpuidFeature, Decoder, DecoderOptions};
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(FromArgs)]
/// identify the ABI extentions in use in a binary.
struct Arghs {
    /// path to the binary object.
    #[argh(positional, from_str_fn(into_pathbuf))]
    path: PathBuf,
}

// `Argh` parse helper function to parse the argument into a `PathBuf`.
fn into_pathbuf(s: &str) -> Result<PathBuf, String> {
    // `PathBuf::from_str` is infallible, therefore unwrap is fine and we will never return `String`.
    Ok(PathBuf::from_str(s).unwrap())
}

fn get_elf_data_from_path(p: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut elf_file = File::open(p)?;
    let mut elf_buf = Vec::<u8>::new();
    elf_file
        .read_to_end(&mut elf_buf)
        .expect("read file failed");
    Ok(elf_buf)
}

fn main() -> Result<(), Box<dyn Error>> {
    let bin: Arghs = argh::from_env();
    let bp = bin.path.as_path();
    let elf_data = get_elf_data_from_path(bp)?;
    let elf = Elf::from_bytes(&elf_data).expect("elf error");

    let bitness = elf.elf_header().class();
    let bitness: u32 = match bitness {
        ElfClass::Elf32 => 32,
        ElfClass::Elf64 => 64,
        ElfClass::Unknown(b) if b == 16 => 16,
        ElfClass::Unknown(_) => return Err(String::from("unknown binary class").into()),
    };

    let text_section = elf
        .lookup_section(b".text")
        .expect("could not lookup .text section");

    let text_section = text_section.content();

    // TODO: Decoder options
    let mut decoder = Decoder::new(bitness, text_section, DecoderOptions::NONE);
    let mut features: HashSet<CpuidFeature> = HashSet::default();

    for ins in decoder.iter() {
        for feat in ins.cpuid_features().iter() {
            features.insert(*feat);
        }
    }

    features.iter().for_each(|f| {
        print!("{:?}  ", f);
    });
    println!();

    Ok(())
}
