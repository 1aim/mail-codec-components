use chrono;
use soft_ascii_string::SoftAsciiString;

use core::utils::HeaderTryFrom;
use core::error::Result;
use core::codec::{EncodeHandle, EncodableInHeader};

/// A DateTime header component wrapping chrono::DateTime<chrono::Utc>
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DateTime( chrono::DateTime<chrono::Utc> );

impl DateTime {

    /// create a new DateTime of the current Time
    pub fn now() -> DateTime {
        DateTime( chrono::Utc::now() )
    }

    /// create a new DateTime from a `chrono::DateTime<TimeZone>` for any `TimeZone`
    pub fn new<TZ: chrono::TimeZone>( date_time: chrono::DateTime<TZ>) -> DateTime {
        DateTime( date_time.with_timezone( &chrono::Utc ) )
    }

    #[doc(hidden)]
    #[cfg(test)]
    pub fn test_time( modif: u32 ) -> Self {
        use chrono::prelude::*;
        Self::new( FixedOffset::east( 3 * 3600 ).ymd( 2013, 8, 6 ).and_hms( 7, 11, modif ) )
    }
}

impl EncodableInHeader for DateTime {

    fn encode(&self, handle: &mut EncodeHandle) -> Result<()> {
        let time = SoftAsciiString::from_string_unchecked(self.to_rfc2822());
        handle.write_str( &*time )?;
        Ok( () )
    }

    fn boxed_clone(&self) -> Box<EncodableInHeader> {
        Box::new(self.clone())
    }
}

impl<TZ> HeaderTryFrom<chrono::DateTime<TZ>> for DateTime
    where TZ: chrono::TimeZone
{
    fn try_from(val: chrono::DateTime<TZ>) -> Result<Self> {
        Ok(Self::new(val))
    }
}

impl<TZ> From<chrono::DateTime<TZ>> for DateTime
    where TZ: chrono::TimeZone
{
    fn from(val: chrono::DateTime<TZ>) -> Self {
        Self::new(val)
    }
}

deref0!{-mut DateTime => chrono::DateTime<chrono::Utc> }


#[cfg(test)]
mod test {
    use super::DateTime;

    ec_test!{ date_time, {
        DateTime::test_time( 45 )
    } => ascii => [
        Text "Tue,  6 Aug 2013 04:11:45 +0000"
    ]}

}