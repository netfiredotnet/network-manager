use std::rc::Rc;
use errors::*;
use dbus_nm::DBusNetworkManager;
use connection::{Connection, ConnectionState, wait};
use device::{Device, PathGetter};

pub struct EthernetDevice<'a> {
    dbus_manager: Rc<DBusNetworkManager>,
    device: &'a Device,
}

impl<'a> EthernetDevice<'a> {

    pub fn set_dhcp(&self) -> Result<(Connection, ConnectionState)> {
        let (path, _) = &self.dbus_manager.create_dhcp_config(self.device.path())?;

        let connection = Connection::init(&self.dbus_manager, &path)?;

        let state = wait(
            &connection,
            &ConnectionState::Activated,
            self.dbus_manager.method_timeout(),
        )?;

        Ok((connection, state))
    }
}


pub fn new_ethernet_device<'a>(
    dbus_manager: &Rc<DBusNetworkManager>,
    device: &'a Device,
) -> EthernetDevice<'a> {
    EthernetDevice {
        dbus_manager: Rc::clone(dbus_manager),
        device: device,
    }
}