use std::collections::{BTreeMap, HashMap};
use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::io::SeekFrom::Start;
use std::ops::Range;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use crate::error::KvError;
use crate::kv::Command::{Remove, Set};
use super::Result;

pub struct KvStore {
    path: PathBuf,
    // map: HashMap<String, String>,
    gen: u64,
    readers: HashMap<u64, BufReaderWithPos<File>>,
    writer: BufWriterWithPos<File>,
    index: BTreeMap<String, CommandPos>,
    uncompacted: u64,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        let mut readers = HashMap::new();
        let mut index: BTreeMap<String, CommandPos> = BTreeMap::new();
        let mut uncompacted = 0;
        let gens = sort_gen(&path)?;

        for &gen in &gens {
            let mut reader = BufReaderWithPos::new(File::open(format!("{}/{}.log", path.to_str().unwrap().to_string(), gen))?)?;
            uncompacted += load(gen, &mut reader, &mut index)?;
            readers.insert(gen, reader);
        }

        let current_gen = gens.last().unwrap_or(&0) + 1;
        let writer = new_log_file(current_gen, &path, &mut readers)?;

        Ok(KvStore {
            path,
            gen: current_gen,
            readers,
            writer,
            index,
            uncompacted,
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        let pos = self.writer.pos;
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;

        if let Command::Set { key, .. } = cmd {
            if let Some(old_cmd) = self.index.insert(key, (self.gen, pos..self.writer.pos).into()) {
                self.uncompacted += old_cmd.len;
            }
        }

        if self.uncompacted > (1024 * 1024) {
            self.compacted()?;
        }

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(&key) {
            let reader = self.readers.get_mut(&cmd_pos.gen).expect("s");

            reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            let cmd_reader = reader.take(cmd_pos.len);

            if let Set { value, .. } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                Err(KvError::UnexpectedCommandType)
            }
        } else {
            Ok(None)
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if let Some(cmd_pos) = self.index.remove(&key) {
            let cmd = Command::remove(key);
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;
            self.uncompacted += cmd_pos.len;
            Ok(())
        } else {
            Err(KvError::KeyNotFound)
        }
    }

    pub fn compacted(&mut self) -> Result<()> {
        let compacted_gen = self.gen + 1;
        self.gen += 2;
        self.writer = self.new_log_file(self.gen)?;

        let mut writer_compacted = self.new_log_file(compacted_gen)?;
        let mut pos = 0;
        for cmd in self.index.values_mut() {
            let reader = self.readers.get_mut(&cmd.gen).unwrap();

            if reader.pos != cmd.pos {
                reader.seek(Start(cmd.pos))?;
            }
            let mut take = reader.take(cmd.len);
            io::copy(&mut take, &mut writer_compacted)?;
            *cmd = (compacted_gen, pos..pos + cmd.len).into();

            pos = pos + cmd.len;
        }

        let need_remove_keys: Vec<u64> = self.readers.keys()
            .filter(|&&it| it < compacted_gen)
            .cloned()
            .collect();

        for need_remove_gen in need_remove_keys {
            self.readers.remove(&need_remove_gen);
            fs::remove_file(log_path(need_remove_gen, self.path.as_path()))?;
        }
        self.uncompacted = 0;
        Ok(())
    }

    pub fn new_log_file(&mut self, gen: u64) -> Result<BufWriterWithPos<File>> {
        new_log_file(gen, self.path.as_path(), &mut self.readers)
    }
}

fn sort_gen(path: &Path) -> Result<Vec<u64>> {
    let mut x: Vec<u64> = fs::read_dir(path)?
        .flat_map(|it| {
            return Result::Ok(it?.path());
        })
        .filter(|it| {
            // println!("isFile: {} extension: {:?}", it.is_file(), it.extension());
            it.is_file() && it.extension() == Some("log".as_ref())
        })
        .flat_map(|it| {
            it
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();

    x.sort_unstable();

    Ok(x)
}

fn load(
    gen: u64,
    reader: &mut BufReaderWithPos<File>,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    let mut uncompacted = 0;
    while let Some(command) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match command? {
            Command::Set { key, .. } => {
                if let Some(old_cmd) = index.insert(key, (gen, pos..new_pos).into()) {
                    uncompacted += old_cmd.len;
                }
            }
            Command::Remove { key } => {
                if let Some(cmd) = index.remove(&key) {
                    uncompacted += cmd.len;
                }

                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    }

    Ok(uncompacted)
}

fn new_log_file(gen: u64, path: &Path, readers: &mut HashMap<u64, BufReaderWithPos<File>>) -> Result<BufWriterWithPos<File>> {
    let new_log_path = log_path(gen, path);
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(new_log_path)?;

    readers.insert(gen, BufReaderWithPos::new(file.try_clone()?)?);

    return BufWriterWithPos::new(file);
}

fn log_path(gen: u64, dir: &Path) -> PathBuf {
    dir.join(format!("{}.log", gen))
}

#[derive(Serialize, Deserialize)]
struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            gen,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Set { key, value }
    }

    fn remove(key: String) -> Command {
        Remove { key }
    }
}

struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos<T: Write + Seek> {
    writer: BufWriter<T>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}
