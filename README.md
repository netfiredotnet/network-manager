# network-manager

> Rust NetworkManager bindings

[![Current Release](https://img.shields.io/github/release/balena-io-modules/network-manager.svg?style=flat-square)](https://github.com/balena-io-modules/network-manager/releases/latest)
[![License](https://img.shields.io/github/license/balena-io-modules/network-manager.svg?style=flat-square)](https://github.com/balena-io-modules/network-manager/blob/master/LICENSE)
[![Issues](https://img.shields.io/github/issues/balena-io-modules/network-manager.svg?style=flat-square)](https://github.com/balena-io-modules/network-manager/issues)

<div align="center">
  <sub>an open source :satellite: project by <a href="https://balena.io">balena.io</a></sub>
</div>

---

## What is Ember?

Ember is a versatile edge device offered by [NetFire](https://netfire.com) that enables a wide range of capabilities for its clients, including:

- **Digital signage** - Display dynamic content on screens
- **Failover internet connection control** - Automatic network failover and management
- **Network and server monitoring** - Real-time visibility into infrastructure health
- **Scan-to-cloud** - Seamless document digitization workflows
- And more...

Head to [netfire.com](https://netfire.com) to get started.

---

## Fork Features

This fork adds the ability to reset wired ethernet connections to DHCP addressing.

### Reset Ethernet to DHCP

```rust
use network_manager::NetworkManager;

let manager = NetworkManager::new();

for device in manager.get_devices().unwrap() {
    if let Some(ethernet) = device.as_ethernet_device() {
        // Reset the ethernet connection to use DHCP
        let (connection, state) = ethernet.set_dhcp().unwrap();
        println!("Connection activated: {:?}", state);
    }
}
```

---

## Support

If you're having any problem, please [raise an issue](https://github.com/balena-io-modules/network-manager/issues/new) on GitHub or [contact us](https://balena.io/community/), and the balena.io team will be happy to help.

---

## License

network-manager is free software, and may be redistributed under the terms specified in
the [license](https://github.com/balena-io-modules/network-manager/blob/master/LICENSE).
