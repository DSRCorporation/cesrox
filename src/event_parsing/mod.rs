use base64::URL_SAFE_NO_PAD;

use self::payload_size::PayloadType;

pub mod attachment;
pub mod payload_size;
pub mod prefix;

pub(crate) fn pack_sn(sn: u64) -> String {
    let payload_type = PayloadType::OA;
    let sn_raw: Vec<u8> = sn.to_be_bytes().into();
    // Calculate how many zeros are missing to achieve expected base64 string
    // length. Master code size is expected padding size.
    let missing_zeros =
        payload_type.size() / 4 * 3 - payload_type.master_code_size(false) - sn_raw.len();
    let sn_vec: Vec<u8> = std::iter::repeat(0)
        .take(missing_zeros)
        .chain(sn_raw)
        .collect();
    [
        payload_type.to_string(),
        base64::encode_config(sn_vec, URL_SAFE_NO_PAD),
    ]
    .join("")
}

pub(crate) fn pack_counter(payload_type: PayloadType, len: usize, data: String) -> String {
    [payload_type.adjust_with_num(len as u16), data].join("")
}
