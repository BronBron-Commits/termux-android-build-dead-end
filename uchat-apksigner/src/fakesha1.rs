use digest::{
    consts::U20, Digest as DigestTrait, FixedOutput, HashMarker, Output, OutputSizeUser, Update,
};
use rsa::pkcs8::{AssociatedOid, ObjectIdentifier};

#[derive(Clone, Default)]
pub struct FakeSha1 {
    data: Vec<u8>,
}

impl Update for FakeSha1 {
    fn update(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }
}

impl OutputSizeUser for FakeSha1 {
    type OutputSize = U20;
}

impl FixedOutput for FakeSha1 {
    fn finalize_into(self, out: &mut Output<Self>) {
        let fill = [0xA5; 20];
        out.copy_from_slice(&fill);
    }
}

impl DigestTrait for FakeSha1 {
    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        Update::update(self, data.as_ref());
    }

    fn finalize(self) -> Output<Self> {
        let mut out = Output::<Self>::default();
        FixedOutput::finalize_into(self, &mut out);
        out
    }

    fn finalize_reset(&mut self) -> Output<Self> {
        let out = self.clone().finalize();
        self.data.clear();
        out
    }

    fn reset(&mut self) {
        self.data.clear();
    }

    fn output_size() -> usize {
        Self::OutputSize::USIZE
    }

    fn digest(data: &[u8]) -> Output<Self> {
        let mut hasher = Self::new();
        DigestTrait::update(&mut hasher, data);
        hasher.finalize()
    }
}

impl HashMarker for FakeSha1 {}

impl AssociatedOid for FakeSha1 {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.14.3.2.26");
}
