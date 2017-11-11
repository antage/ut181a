pub(crate) struct Packet {
    data: Vec<u8>
}

impl Packet {
    pub(crate) fn checksum(&self) -> u16 {
        let mut sum = 2 + (self.data.len() & 0xFF) as u16 + ((self.data.len() >> 8) & 0xFF) as u16;
        for b in &self.data {
            sum = sum.wrapping_add(u16::from(*b));
        }
        sum
    }

    pub(crate) fn new(body: &[u8]) -> Packet {
        let mut data = Vec::with_capacity(body.len());
        data.extend(body);
        Packet {
            data,
        }
    }

    pub(crate) fn frame(&self) -> Vec<u8> {
        let mut pkt: Vec<u8> = Vec::with_capacity(6 + self.data.len());

        pkt.push(0xAB);
        pkt.push(0xCD);

        let pkt_len = self.data.len() + 2;

        pkt.push((pkt_len & 0xFF) as u8);
        pkt.push(((pkt_len >> 8) & 0xFF) as u8);

        pkt.extend(&self.data);

        let cs = self.checksum();

        pkt.push((cs & 0xFF) as u8);
        pkt.push(((cs >> 8) & 0xFF) as u8);

        pkt
    }
}
