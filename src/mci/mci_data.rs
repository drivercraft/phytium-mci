use alloc::vec::Vec;
#[cfg(feature = "dma")]
use dma_api::DVec;

pub(crate) struct MCIData {
    #[cfg(feature = "pio")]
    buf: Option<Vec<u32>>,
    #[cfg(feature = "dma")]
    buf_dma: Option<DVec<u32>>,
    blksz: u32,   // 块大小
    blkcnt: u32,  // 块数量
    datalen: u32, // 数据长度
}

impl MCIData {
    pub(crate) fn new() -> Self {
        MCIData {
            #[cfg(feature = "pio")]
            buf: None,
            #[cfg(feature = "dma")]
            buf_dma: None,
            blksz: 0,
            blkcnt: 0,
            datalen: 0,
        }
    }

    pub(crate) fn blksz(&self) -> u32 {
        self.blksz
    }

    pub(crate) fn blksz_set(&mut self, blksz: u32) {
        self.blksz = blksz
    }

    pub(crate) fn blkcnt(&self) -> u32 {
        self.blkcnt
    }

    pub(crate) fn blkcnt_set(&mut self, blkcnt: u32) {
        self.blkcnt = blkcnt
    }

    pub(crate) fn datalen(&self) -> u32 {
        self.datalen
    }

    pub(crate) fn datalen_set(&mut self, datalen: u32) {
        self.datalen = datalen
    }

    #[cfg(feature = "pio")]
    pub(crate) fn buf(&self) -> Option<&Vec<u32>> {
        self.buf.as_ref()
    }

    #[cfg(feature = "pio")]
    pub(crate) fn buf_mut(&mut self) -> Option<&mut Vec<u32>> {
        self.buf.as_mut()
    }

    #[cfg(feature = "pio")]
    pub(crate) fn buf_set(&mut self, buf: Option<Vec<u32>>) {
        self.buf = buf
    }

    #[cfg(feature = "dma")]
    pub(crate) fn buf_dma(&self) -> Option<&DVec<u32>> {
        self.buf_dma.as_ref()
    }

    #[cfg(feature = "dma")]
    pub(crate) fn buf_dma_set(&mut self, buf_dma: Option<DVec<u32>>) {
        self.buf_dma = buf_dma;
    }

    #[cfg(feature = "dma")]
    #[allow(dead_code)]
    pub fn dma_to_vec(&self) -> Vec<u32> {
        let dvec: &DVec<u32> = self.buf_dma.as_ref().unwrap();
        (**dvec).to_vec()
    }

    #[cfg(feature = "dma")]
    pub(crate) fn take_buf_dma(&mut self) -> Option<DVec<u32>> {
        self.buf_dma.take()
    }
}
