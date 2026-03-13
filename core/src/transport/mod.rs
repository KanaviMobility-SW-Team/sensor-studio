use bytes::Bytes;

pub type TransportId = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportKind {
    Udp,
    Tcp,
    Usb,
    Serial,
}

pub trait Transport {
    fn id(&self) -> &str;

    fn kind(&self) -> TransportKind;

    fn read_chunk(&mut self) -> Option<Bytes>;
}
