#![no_main]

use ecdsa::elliptic_curve::dev::MockCurve;
use libfuzzer_sys::fuzz_target;
type Signature = ecdsa::der::Signature<MockCurve>;

fuzz_target!(|data: &[u8]| {
    let sig = Signature::from_bytes(data);
    if sig.is_ok() {
        let sig = sig.unwrap();
        let data2: &[u8] = sig.as_bytes();
        assert_eq!(data2, data);
    }
});
