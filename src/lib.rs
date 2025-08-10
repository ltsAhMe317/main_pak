use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};

const PAK_SPLIT: &str = "[MAIN_PAK]";
pub struct Pak {
    pub date: Vec<(PathBuf, Vec<u8>)>,
}
impl Pak {
    pub fn save(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        // 直接创建压缩编码器写入文件
        let file = std::fs::File::create(path)?;
        let mut encoder = ZlibEncoder::new(file, Compression::best());

        for (i, (path, date)) in self.date.iter().enumerate() {
            // 处理路径转换错误（替代unwrap）
            let path_str = path.to_str().ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Path contains invalid UTF-8",
                )
            })?;

            // 使用长度前缀替代分隔符
            encoder.write_all(PAK_SPLIT.as_bytes())?;
            encoder.write_all(path_str.as_bytes())?;

            encoder.write_all(PAK_SPLIT.as_bytes())?;
            encoder.write_all(date)?;
        }

        // 确保压缩完成
        encoder.finish()?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Self {
        let mut encoder = ZlibDecoder::new(fs::File::open(path).unwrap());

        let mut read = Vec::new();
        encoder.read_to_end(&mut read).unwrap();

        let check_str = unsafe { String::from_utf8_unchecked(read) };
        dbg!(&check_str);
        let read = check_str.as_bytes();
        let mut map = Vec::new();
        let mut start: usize = 0;
        let mut path = None;
        let mut date = None;
        while let Some(index) = check_str[start..].find(PAK_SPLIT) {
            start = index + PAK_SPLIT.len() + start;
            println!("{start}");
            {
                let next = if let Some(next) = check_str[start..].find(PAK_SPLIT) {
                    next + start
                } else {
                    check_str.len()
                };

                if path.is_none() {
                    path = Some(read[start..next].to_vec());
                } else if date.is_none() {
                    date = Some(read[start..next].to_vec());
                }
                if path.is_some() && date.is_some() {
                    map.push((
                        Path::new(&String::from_utf8(path.take().unwrap()).unwrap()).to_path_buf(),
                        date.take().unwrap(),
                    ));
                }
            }
        }
        Self { date: map }
    }

    fn add_files(&mut self, files: HashMap<PathBuf, Vec<u8>>) {
        self.date.extend(files);
    }
    pub fn group_dir(&self, path: impl AsRef<Path>) -> Vec<&(PathBuf, Vec<u8>)> {
        let path = path.as_ref().to_str().unwrap();
        dbg!(path);
        let mut vec = Vec::new();
        for file @ (buf, _) in self.date.iter() {
            if &buf.to_str().unwrap()[..path.len()] == path {
                vec.push(file);
            }
        }
        vec
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

pub fn pak_path(path: impl AsRef<Path>) -> Pak {
    //end
    let mut pak = Pak { date: Vec::new() };
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
}
