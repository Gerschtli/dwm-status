use std::fs;
use std::io;
use std::io::Read;
use std::str;

pub(crate) fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub(crate) fn read_int_from_file<T>(path: &str) -> io::Result<T>
where
    T: str::FromStr,
{
    read_file(path)?
        .trim_right_matches('\n')
        .parse()
        .or_else(|_| {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("file \"{}\" doesn't contain an int value", &path),
            ))
        })
}
