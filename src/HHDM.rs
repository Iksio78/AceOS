use limine::request::HhdmRequest;

pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

static mut HHDM_OFFSET: u64 = 0;

pub fn init() -> Option<()> {
    let response = HHDM_REQUEST.response()?;

    unsafe {
        HHDM_OFFSET = response.offset;
    }

    Some(())
}

pub fn offset() -> u64 {
    unsafe { HHDM_OFFSET }
}

pub fn phys_to_virt(phys: u64) -> u64 {
    offset() + phys
}