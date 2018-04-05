use soft_ascii_string::SoftAsciiChar;

use core::error::Result;
use core::codec::{EncodableInHeader, EncodeHandle};
use super::word::{ Word, do_encode_word };
use super::{ Email, Domain };


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ReceivedToken {
    Word( Word ),
    Address( Email ),
    Domain( Domain )
}

impl EncodableInHeader for  ReceivedToken {

    fn encode(&self, handle: &mut EncodeHandle) -> Result<()> {
        use self::ReceivedToken::*;
        match *self {
            Word( ref word ) => {
                do_encode_word( word, handle, None )?;
            },
            Address( ref addr ) => {
                // we do not need to use <..> , but I think it's better and it is definitely
                // not wrong
                handle.write_char( SoftAsciiChar::from_char_unchecked('<') )?;
                addr.encode( handle )?;
                handle.write_char( SoftAsciiChar::from_char_unchecked('>') )?;
            },
            Domain( ref domain ) => {
                domain.encode( handle )?;
            }
        }
        Ok( () )
    }

    fn boxed_clone(&self) -> Box<EncodableInHeader> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod test {
    use core::utils::HeaderTryFrom;
    use core::MailType;
    use core::codec::{Encoder, VecBodyBuf};
    use super::*;

    ec_test!{ a_domain, {
        Domain::try_from( "random.mailnot" )?
    } => ascii => [
        MarkFWS,
        Text "random.mailnot",
        MarkFWS
    ]}

    ec_test!{ a_address, {
        let email = Email::try_from( "modnar@random.mailnot")?;
        ReceivedToken::Address( email )
    } => ascii => [
        Text "<",
        MarkFWS,
        Text "modnar",
        MarkFWS,
        Text "@",
        MarkFWS,
        Text "random.mailnot",
        MarkFWS,
        Text ">"
    ]}

    ec_test!{ a_word, {
        let word = Word::try_from( "simple" )?;
        ReceivedToken::Word( word )
    } => ascii => [
        Text "simple"
    ]}

    ec_test!{ a_quoted_word, {
        let word = Word::try_from( "sim ple" )?;
        ReceivedToken::Word( word )
    } => ascii => [
        Text r#""sim ple""#
    ]}


    #[test]
    fn no_encoded_word() {
        let mut encoder = Encoder::<VecBodyBuf>::new( MailType::Ascii );
        let mut handle = encoder.encode_handle();
        let input = ReceivedToken::Word( Word::try_from( "↓right" ).unwrap() );
        assert_err!(input.encode( &mut handle ));
        handle.undo_header();
    }
}