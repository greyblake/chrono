#![cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]

use super::{DateTime, NaiveDateTime, TimeZone};
use arbitrary::{Arbitrary, Unstructured};

// NOTE: Implementation of Arbitrary cannot be simply derived for DateTime<Tz>, due to
// the nontrivial bound <Tz as TimeZone>::Offset: Arbitrary.

#[cfg(feature = "arbitrary")]
impl<'a, Tz> Arbitrary<'a> for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<DateTime<Tz>> {
        let datetime = NaiveDateTime::arbitrary(u)?;
        let offset = <Tz as TimeZone>::Offset::arbitrary(u)?;
        Ok(DateTime::from_utc(datetime, offset))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::offset::FixedOffset;

    const UNSTRCUTURED_BINARY1: [u8; 8] = [0x8f, 0xc2, 0x95, 0xdc, 0x3e, 0x45, 0xb2, 0x3e];
    const UNSTRCUTURED_BINARY2: [u8; 8] = [0x8b, 0xad, 0x2c, 0xc9, 0xf0, 0x05, 0x75, 0x84];

    #[test]
    fn test_different_unstructured() {
        let mut unstrctured1 = Unstructured::new(&UNSTRCUTURED_BINARY1);
        let datetime1: DateTime<FixedOffset> = DateTime::arbitrary(&mut unstrctured1).unwrap();

        let mut unstrctured2 = Unstructured::new(&UNSTRCUTURED_BINARY2);
        let datetime2: DateTime<FixedOffset> = DateTime::arbitrary(&mut unstrctured2).unwrap();

        assert_ne!(datetime1, datetime2);
    }

    #[test]
    fn test_same_unstructured() {
        let mut unstrctured1 = Unstructured::new(&UNSTRCUTURED_BINARY1);
        let datetime1: DateTime<FixedOffset> = DateTime::arbitrary(&mut unstrctured1).unwrap();

        let mut unstrctured2 = Unstructured::new(&UNSTRCUTURED_BINARY1);
        let datetime2: DateTime<FixedOffset> = DateTime::arbitrary(&mut unstrctured2).unwrap();

        assert_eq!(datetime1, datetime2);
    }
}
