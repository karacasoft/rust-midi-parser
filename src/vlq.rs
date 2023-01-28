
#[macro_export]
macro_rules! u32_from_vlq {
    ( $array: expr ) => {
        {
            use crate::vlq::Vlq;
            let my_vlq = Vlq::from($array);
            u32::from(my_vlq)
        }
    };
}

pub struct Vlq([u8; 4]);

impl From<[u8; 4]> for Vlq {
    fn from(bytes: [u8; 4]) -> Vlq {
        return Vlq(bytes);
    }
}

impl From<Vlq> for u32 {
    fn from(vlq: Vlq) -> u32 {
        let mut result;
        if vlq.0[0] <= 127 {
            return vlq.0[0] as u32;
        } else {
            result = ((vlq.0[0] & 0b01111111) as u32) << 7;
        }

        if vlq.0[1] <= 127 {
            return result | (vlq.0[1] as u32);
        } else {
            result = result | ((vlq.0[1] & 0b01111111) as u32);
            result = result << 7;
        }

        if vlq.0[2] <= 127 {
            return result | (vlq.0[2] as u32);
        } else {
            result = result | ((vlq.0[2] & 0b01111111) as u32);
            result = result << 7;
        }
        
        return result | ((vlq.0[3] & 0b01111111) as u32);
    }
}