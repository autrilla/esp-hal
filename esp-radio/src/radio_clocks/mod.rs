#[cfg_attr(esp32, path = "clocks_ll/esp32.rs")]
#[cfg_attr(esp32c2, path = "clocks_ll/esp32c2.rs")]
#[cfg_attr(esp32c3, path = "clocks_ll/esp32c3.rs")]
#[cfg_attr(esp32c5, path = "clocks_ll/esp32c5.rs")]
#[cfg_attr(esp32c6, path = "clocks_ll/esp32c6.rs")]
#[cfg_attr(esp32c61, path = "clocks_ll/esp32c61.rs")]
#[cfg_attr(esp32h2, path = "clocks_ll/esp32h2.rs")]
#[cfg_attr(esp32s2, path = "clocks_ll/esp32s2.rs")]
#[cfg_attr(esp32s3, path = "clocks_ll/esp32s3.rs")]
#[cfg_attr(esp32s31, path = "clocks_ll/esp32s31.rs")]
#[allow(unused)]
pub(crate) mod clocks_ll;

#[inline]
/// Do any common initialization needed for the radio clocks
pub(crate) fn init_radio_clocks() {
    clocks_ll::init_clocks();
}

/// Undo the clock initialization done by [`init_radio_clocks`], gating the
/// modem clocks again (mirroring ESP-IDF's per-module clock disable).
#[inline]
pub(crate) fn deinit_radio_clocks() {
    clocks_ll::deinit_clocks();
}

/// Counts the users of a clock. It switches the clock on for the first user and off after the
/// last user.
#[allow(unused)]
pub(crate) struct Refcount {
    count: esp_sync::NonReentrantMutex<usize>,
}

#[allow(unused)]
impl Refcount {
    pub(crate) const fn new() -> Self {
        Self {
            count: esp_sync::NonReentrantMutex::new(0),
        }
    }

    pub(crate) fn update(&self, en: bool, f: fn(bool)) {
        self.count.with(|refcount| {
            let count = *refcount;
            let run = if en {
                *refcount = count + 1;
                count == 0
            } else {
                // A radio deinit can disable a clock that it did not enable.
                *refcount = count.saturating_sub(1);
                count == 1
            };
            if run {
                f(en);
            }
        });
    }
}
