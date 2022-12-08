pub(crate) mod bcm2xxx_gpio;
pub(crate) mod bcm2xxx_qa7;
pub(crate) mod bcm2xxx_systimer;
pub(crate) mod bcm2837_mini_uart;
pub(crate) mod common;

use crate::synchronization::{SpinLock, interface::Mutex};

static DRIVER_MANAGER: DriverManager = DriverManager::new();

const NUM_DRIVERS: usize = 0;

struct DriverManager {
    inner: SpinLock<DriverManagerInner>,
}

impl DriverManager {
    const fn new() -> Self {
        Self {
            inner: SpinLock::new(DriverManagerInner::new()),
        }
    }

    pub fn register_driver(&self, descriptor: DeviceDriverDescriptor) {
        let mut inner = self.inner.lock().unwrap();
        let idx = inner.next_index;
        inner.descriptors[idx] = Some(descriptor);
        inner.next_index += 1;
    }

    fn for_each_descriptor(&self, f: impl FnMut(& DeviceDriverDescriptor)) {
        let mut inner = self.inner.lock().unwrap();
        inner.descriptors.iter().filter_map(|x| x.as_ref()).for_each(f);
    }

    pub unsafe fn init_drivers(&self) {
        self.for_each_descriptor(|descriptor| {
            // 1. Initialize driver.
            if let Err(x) = descriptor.device_driver.init() {
                panic!(
                    "Error initializing driver: {}: {}",
                    descriptor.device_driver.compatible(),
                    x
                );
            }

            // 2. Call corresponding post init callback.
            if let Some(callback) = &descriptor.post_init_callback {
                if let Err(x) = callback() {
                    panic!(
                        "Error during driver post-init callback: {}: {}",
                        descriptor.device_driver.compatible(),
                        x
                    );
                }
            }
        });
    }

    pub fn enumerate(&self) {
        let mut i: usize = 1;
        self.for_each_descriptor(|descriptor| {
            crate::println!("      {}. {}", i, descriptor.device_driver.compatible());

            i += 1;
        });
    }
}

struct DriverManagerInner {
    next_index: usize,
    descriptors: [Option<DeviceDriverDescriptor>; NUM_DRIVERS],
}

impl DriverManagerInner {
    pub const fn new() -> Self {
        Self {
            next_index: 0,
            descriptors: [None; NUM_DRIVERS],
        }
    }
}

pub mod interface {
    pub trait DeviceDriver {
        fn compatible(&self) -> &'static str;

        unsafe fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }
}

pub type DeviceDriverPostInitCallback = unsafe fn() -> Result<(), &'static str>;

#[derive(Copy, Clone)]
pub struct DeviceDriverDescriptor {
    device_driver: &'static (dyn interface::DeviceDriver + Sync),
    post_init_callback: Option<DeviceDriverPostInitCallback>,
}

