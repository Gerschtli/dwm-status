use std::fs;
use std::io;
use std::io::Read;
use std::path;
use std::str;

pub(crate) fn read<P>(path: P) -> io::Result<String>
where
    P: AsRef<path::Path>,
{
    let mut s = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg_attr(all(test, feature = "mocking"), mocktopus::macros::mockable)]
pub(crate) fn parse_file_content<P, T>(path: P) -> io::Result<T>
where
    P: AsRef<path::Path>,
    T: str::FromStr,
{
    read(&path)?.trim().parse().map_err(|_| {
        io::Error::new(
            io::ErrorKind::Other,
            format!(
                "contents of file '{}' failed to parse",
                path.as_ref().display()
            ),
        )
    })
}
