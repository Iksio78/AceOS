use limine::request::MemmapRequest;

use crate::{HHDM, bitmap::Bitmap, println};


pub struct PMM{
    pages: Bitmap<'static>,
    pub total_pages: usize,
    pub free_pages: usize,
    first_free: usize
}
impl PMM{
    pub fn init(memmap_req: &MemmapRequest) -> Option<Self>{
        let mut p: Bitmap<'static>;
        let mut tp: usize = 0;
        let mut fp: usize = 0;
        let mut ff: usize = 0;
        

        if let Some(memmap_resp) = memmap_req.response() {
            for e in memmap_resp.entries() {
                let end = e.base as usize + e.length as usize;
                tp = tp.max((end + 4095) / 4096);
            }
        }

        let bitmap_size = (tp + 7) / 8;
        let bitmap_pages = (bitmap_size + 4095) / 4096;
        let bitmap_size_aligned = bitmap_pages * 4096;

        let mut bitmap_phys = None;

        if let Some(memmap_resp) = memmap_req.response() {
            for e in memmap_resp.entries() {
                if e.type_ == 0 && e.length as usize >= bitmap_size_aligned{
                    bitmap_phys = Some(e.base as usize);
                    break;
                }
            }
        }

        let bitmap_phys = bitmap_phys?;

        let bitmap_virt = bitmap_phys + HHDM::offset() as usize; 

        let bitmap_data = unsafe {
          core::slice::from_raw_parts_mut(
            bitmap_virt as *mut u8,
            bitmap_size_aligned,
            )
        };

        p = Bitmap::new(bitmap_data);

        if let Some(memmap_resp) = memmap_req.response() {
            for e in memmap_resp.entries() {
                if e.type_ == 0{
                    let start = e.base as usize / 4096;
                    let end = (e.base as usize + e.length as usize + 4095) / 4096;
                    
                    
                    for page in start..end {
                        p.set(page, false);
                    }
                }
            }
        }
        let bitmap_start = bitmap_phys / 4096;

        for page in bitmap_start..bitmap_start + bitmap_pages {
            p.set(page, true);
        }

        ff = p.find_first_zero().unwrap_or(0);

        fp = tp;
        for i in 0..tp{
            fp -= p.get(i) as usize;
        }

        Some(Self {
            pages: p,
            total_pages: tp,
            free_pages: fp,
            first_free: ff,
        })
    }

    pub fn alloc_page(&mut self) -> Option<usize>{
        if self.free_pages == 0 {
            return None;
        }

        let page = self.first_free;

        self.pages.set(page, true);

        self.first_free = self.pages.find_first_zero()
        .unwrap_or(0);

        self.free_pages -= 1;

        Some(page * 4096)
    }

    pub fn free_page(&mut self, address: usize) {
        let page = address / 4096;

        if self.pages.get(page) {
            self.pages.set(page, false);
            self.free_pages += 1;

            if page < self.first_free {
                self.first_free = page;
            }
        }
    } 
}