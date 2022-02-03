#![crate_type = "lib"]

struct Shared {
    big: Option<[u8; 2000]>,
}

struct Local {
    big: Option<[u8; 1000]>,
}

fn init() -> (Shared, Local) {
    (Shared { big: None }, Local { big: None })
}

pub unsafe fn naughty_function() {
    #[inline(never)]
    fn __rtic_init_resources<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
    __rtic_init_resources(|| {
        let (shared_resources, local_resources) = init();
        __rtic_internal_shared_resource_big
            .get_mut()
            .write(core::mem::MaybeUninit::new(shared_resources.big));
        __rtic_internal_local_resource_big
            .get_mut()
            .write(core::mem::MaybeUninit::new(local_resources.big));
    });
}

pub struct RacyCell<T>(core::cell::UnsafeCell<T>);
unsafe impl<T> Sync for RacyCell<T> {}
impl<T> RacyCell<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        RacyCell(core::cell::UnsafeCell::new(value))
    }

    #[inline(always)]
    pub unsafe fn get_mut(&self) -> *mut T {
        self.0.get()
    }
}

static __rtic_internal_local_resource_big: RacyCell<
    core::mem::MaybeUninit<Option<[u8; 1000]>>,
> = RacyCell::new(core::mem::MaybeUninit::uninit());

static __rtic_internal_shared_resource_big: RacyCell<
    core::mem::MaybeUninit<Option<[u8; 2000]>>,
> = RacyCell::new(core::mem::MaybeUninit::uninit());

