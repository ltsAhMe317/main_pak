use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use libdeflater::{CompressionLvl, Compressor, Decompressor};

const PAK_SPLIT: &str = "[MAIN_PAK]";
const PAK_MAX_SIZE_U8: usize = 268435456;

pub struct Paked {
    date: HashMap<PathBuf, Vec<u8>>,
}
impl Paked {
    pub fn save(&self, path: impl AsRef<Path>) {
        let mut compre = Vec::new();
        for (path, date) in self.date.iter() {
            //path
            compre.extend_from_slice(PAK_SPLIT.as_bytes());
            compre.extend_from_slice(path.to_str().unwrap().as_bytes());
            //date
            compre.extend_from_slice(PAK_SPLIT.as_bytes());
            compre.extend_from_slice(date.as_slice());
        }
        fs::write(path, compre).unwrap();
    }
    pub fn load(path: impl AsRef<Path>) -> Self {
        let read = fs::read(path).unwrap();
        let check_str = unsafe { String::from_utf8_unchecked(read) };
        let read = check_str.as_bytes();
        let mut map = HashMap::new();
        let mut start: usize = 0;
        let mut path = None;
        let mut date = None;
        while let Some(index) = check_str[start..].find(PAK_SPLIT) {
            start = index + PAK_SPLIT.len() + start;
            if let Some(next) = check_str[start..].find(PAK_SPLIT) {
                let next = next + start;
                if path.is_some() && date.is_some() {
                    map.insert(
                        Path::new(&String::from_utf8(path.take().unwrap()).unwrap()).to_path_buf(),
                        date.take().unwrap(),
                    );
                }
                if path.is_none() {
                    path = Some(read[start..next].to_vec());
                } else if date.is_none() {
                    date = Some(read[start..next].to_vec());
                }
            }
        }
        Self { date: map }
    }
    fn add_files(&mut self, mut files: HashMap<PathBuf, Vec<u8>>) {
        let mut zip_context = Compressor::new(CompressionLvl::best());
        //zip

        let mut buf = vec![0; PAK_MAX_SIZE_U8];
        for (path, date) in files.iter_mut() {
            let size = zip_context
                .gzip_compress(date.as_slice(), buf.as_mut_slice())
                .unwrap();
            *date = buf[..size].to_vec();
        }
        self.date.extend(files);
    }

    pub fn fast(&self) -> FastPak {
        let mut map = HashMap::new();
        let mut dezip_context = Decompressor::new();
        let mut buf = vec![0; PAK_MAX_SIZE_U8];
        for (path, date) in self.date.iter() {
            let size = dezip_context
                .gzip_decompress(date.as_slice(), buf.as_mut_slice())
                .unwrap();
            map.insert(path.clone(), buf[..size].to_vec());
        }

        FastPak { date: map }
    }
}
pub struct FastPak {
    date: HashMap<PathBuf, Vec<u8>>,
}
impl FastPak {
    pub fn get(&self, path: impl AsRef<Path>) -> Option<&Vec<u8>> {
        self.date.get(path.as_ref())
    }
    pub fn remove(&mut self, path: impl AsRef<Path>) -> Option<Vec<u8>> {
        self.date.remove(path.as_ref())
    }
}

fn read_files(path: impl AsRef<Path>) -> Vec<(PathBuf, Vec<u8>)> {
    if path.as_ref().is_file() {
        panic!("can't read a file!");
    }

    let mut map = Vec::new();

    let path_reader = fs::read_dir(path).unwrap();
    for file in path_reader {
        let file = file.unwrap();
        match file.file_type().unwrap().is_file() {
            true => {
                map.push((file.path(), fs::read(file.path()).unwrap()));
            }
            false => {
                let files = read_files(file.path());
                map.extend(files);
            }
        }
    }
    map
}

pub fn pak_path(path: impl AsRef<Path>) -> Paked {
    //end
    let mut pak = Paked {
        date: HashMap::new(),
    };
    let files = read_files(path.as_ref());

    let path_len = path.as_ref().to_str().unwrap().len();

    let mut map = HashMap::new();
    for (mut path, date) in files {
        path = Path::new(&path.to_str().unwrap()[path_len..]).to_path_buf();
        map.insert(path, date);
    }
    pak.add_files(map);
    pak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pak() {
        pak_path("test_res").save("main.pak");
    }

    #[test]
    fn load() {
        println!(
            "{:?}",
            String::from_utf8_lossy(
                Paked::load("main.pak")
                    .date
                    .get(Path::new("test_res\\the two\\niubi.txt"))
                    .unwrap()
            )
        );
    }
    #[test]
    fn fast() {
        let load = Paked::load("main.pak");
        let mut fast = load.fast();
        println!("{:?}", fast.date.keys());
        println!(
            "{}",
            String::from_utf8(fast.remove("outside.txt").unwrap()).unwrap()
        )
    }
}
