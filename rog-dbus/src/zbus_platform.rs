//! # `DBus` interface proxy for: `xyz.ljones.Asusd`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from `DBus` introspection
//! data. Source: `Interface '/xyz/ljones/Platform' from service
//! 'xyz.ljones.Asusd' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://zeenix.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This `DBus` object implements
//! [standard `DBus` interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use rog_platform::cpu::CPUEPP;
use rog_platform::platform::{PlatformProfile, Properties};
use zbus::proxy;

#[proxy(
    interface = "xyz.ljones.Platform",
    default_service = "xyz.ljones.Asusd",
    default_path = "/xyz/ljones"
)]
pub trait Platform {
    #[zbus(property)]
    fn version(&self) -> zbus::Result<String>;

    /// NextThrottleThermalPolicy method
    fn next_platform_profile(&self) -> zbus::Result<()>;

    /// SupportedProperties method
    fn supported_properties(&self) -> zbus::Result<Vec<Properties>>;

    /// ChargeControlEndThreshold property
    #[zbus(property)]
    fn charge_control_end_threshold(&self) -> zbus::Result<u8>;
    #[zbus(property)]
    fn set_charge_control_end_threshold(&self, value: u8) -> zbus::Result<()>;

    // Toggle one-shot charge to 100%
    fn one_shot_full_charge(&self) -> zbus::Result<()>;

    /// ThrottleBalancedEpp property
    #[zbus(property)]
    fn profile_balanced_epp(&self) -> zbus::Result<CPUEPP>;
    #[zbus(property)]
    fn set_profile_balanced_epp(&self, epp: CPUEPP) -> zbus::Result<()>;

    /// ThrottlePerformanceEpp property
    #[zbus(property)]
    fn profile_performance_epp(&self) -> zbus::Result<CPUEPP>;
    #[zbus(property)]
    fn set_profile_performance_epp(&self, epp: CPUEPP) -> zbus::Result<()>;

    /// ThrottlePolicyLinkedEpp property
    #[zbus(property)]
    fn platform_profile_linked_epp(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_platform_profile_linked_epp(&self, value: bool) -> zbus::Result<()>;

    /// ThrottlePolicyOnAc property
    #[zbus(property)]
    fn platform_profile_on_ac(&self) -> zbus::Result<PlatformProfile>;
    #[zbus(property)]
    fn set_platform_profile_on_ac(&self, platform_profile: PlatformProfile) -> zbus::Result<()>;

    /// ChangeThrottlePolicyOnAc property
    #[zbus(property)]
    fn change_platform_profile_on_ac(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_change_platform_profile_on_ac(&self, change: bool) -> zbus::Result<()>;

    /// ThrottlePolicyOnBattery property
    #[zbus(property)]
    fn platform_profile_on_battery(&self) -> zbus::Result<PlatformProfile>;
    #[zbus(property)]
    fn set_platform_profile_on_battery(
        &self,
        platform_profile: PlatformProfile,
    ) -> zbus::Result<()>;

    /// ChangeThrottlePolicyOnAc property
    #[zbus(property)]
    fn change_platform_profile_on_battery(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_change_platform_profile_on_battery(&self, change: bool) -> zbus::Result<()>;

    /// ThrottleQuietEpp property
    #[zbus(property)]
    fn profile_quiet_epp(&self) -> zbus::Result<CPUEPP>;
    #[zbus(property)]
    fn set_profile_quiet_epp(&self, epp: CPUEPP) -> zbus::Result<()>;

    /// ThrottlePolicy property
    #[zbus(property)]
    fn platform_profile(&self) -> zbus::Result<PlatformProfile>;
    #[zbus(property)]
    fn set_platform_profile(&self, platform_profile: PlatformProfile) -> zbus::Result<()>;

    /// Set if the PPT tuning group for the current profile is enabled
    #[zbus(property)]
    fn enable_ppt_group(&self) -> zbus::Result<bool>;

    /// Set if the PPT tuning group for the current profile is enabled
    #[zbus(property)]
    fn set_enable_ppt_group(&self, enable: bool) -> zbus::Result<()>;
}
