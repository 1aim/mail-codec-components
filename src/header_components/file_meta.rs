
use chrono::DateTime;
use chrono::Utc;

/// A struct representing common file metadata.
///
/// This is used by e.g. attachments, when attaching
/// a file (or embedding an image). Through it's usage
/// is optional.
///
/// # Stability Note
///
/// This is likely to move to an different place at
/// some point, potentially in a different `mail-*`
/// crate.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature="serde-impl", derive(Serialize, Deserialize))]
pub struct FileMeta {
    /// The file name.
    ///
    /// Note that this utility is limited to utf-8 file names.
    /// This is normally used when downloading a attachment to
    /// choose the default file name.
    #[cfg_attr(feature="serde-impl", serde(default))]
    pub file_name: Option<String>,

    /// The creation date of the file (in utc).
    #[cfg_attr(feature="serde-impl", serde(default))]
    #[cfg_attr(feature="serde-impl", serde(deserialize_with = "super::utils::deserialize_opt_time"))]
    #[cfg_attr(feature="serde-impl", serde(serialize_with = "super::utils::serialize_opt_time"))]
    pub creation_date: Option<DateTime<Utc>>,

    /// The last modification date of the file (in utc).
    #[cfg_attr(feature="serde-impl", serde(default))]
    #[cfg_attr(feature="serde-impl", serde(deserialize_with = "super::utils::deserialize_opt_time"))]
    #[cfg_attr(feature="serde-impl", serde(serialize_with = "super::utils::serialize_opt_time"))]
    pub modification_date: Option<DateTime<Utc>>,

    /// The date time the file was read, i.e. placed in the mail (in utc).
    #[cfg_attr(feature="serde-impl", serde(default))]
    #[cfg_attr(feature="serde-impl", serde(deserialize_with = "super::utils::deserialize_opt_time"))]
    #[cfg_attr(feature="serde-impl", serde(serialize_with = "super::utils::serialize_opt_time"))]
    pub read_date: Option<DateTime<Utc>>,

    /// The size the file should have.
    ///
    /// Note that normally mail explicitly opts to NOT specify the size
    /// of a mime-multi part body (e.g. an attachments) and you can never
    /// rely on it to e.g. skip ahead. But it has some uses wrt. thinks
    /// like external headers.
    #[cfg_attr(feature="serde-impl", serde(default))]
    pub size: Option<usize>
}