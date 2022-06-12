#[derive(Debug)]
pub enum Error {
    MissingBegnningLine,
    MissingEndingLine,
    SerdeYaml(serde_yaml::Error),
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::SerdeYaml(error)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Parsed<'a, T: serde::de::DeserializeOwned> {
    pub body: &'a str,
    pub headers: T,
}

static LINE_PATTERN: &str = "---\n";

pub fn parse<T: serde::de::DeserializeOwned>(text: &str) -> Result<Parsed<T>, Error> {
    if !text.starts_with(LINE_PATTERN) {
        return Err(Error::MissingBegnningLine);
    }

    let slice = &text[LINE_PATTERN.len()..];
    let index_of_ending_slice = slice.find(LINE_PATTERN).ok_or(Error::MissingEndingLine)?;
    Ok(Parsed {
        body: &slice[(index_of_ending_slice + LINE_PATTERN.len())..],
        headers: serde_yaml::from_str(&slice[..index_of_ending_slice])?,
    })
}