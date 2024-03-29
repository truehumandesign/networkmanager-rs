// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerVPNConnection {
    fn vpn_state(&self) -> Result<u32, dbus::Error>;
    fn banner(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::Connection>>
    OrgFreedesktopNetworkManagerVPNConnection for blocking::Proxy<'a, C>
{
    fn vpn_state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.VPN.Connection",
            "VpnState",
        )
    }

    fn banner(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.VPN.Connection",
            "Banner",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerVPNConnectionPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerVPNConnectionPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerVPNConnectionPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerVPNConnectionPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerVPNConnectionPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.VPN.Connection";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerVPNConnectionVpnStateChanged {
    pub state: u32,
    pub reason: u32,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerVPNConnectionVpnStateChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.state, i);
        arg::RefArg::append(&self.reason, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerVPNConnectionVpnStateChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerVPNConnectionVpnStateChanged {
            state: i.read()?,
            reason: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerVPNConnectionVpnStateChanged {
    const NAME: &'static str = "VpnStateChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.VPN.Connection";
}
