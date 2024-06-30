use pnet::datalink;


pub fn get_mac_address() -> Option<String> {
    // Iterate over all network interfaces
    for iface in datalink::interfaces() {
        // Check if the interface is up and not a loopback
        if iface.is_up() && !iface.is_loopback() {
            // Get the MAC address if available
            if let Some(mac) = iface.mac {
                return Some(mac.to_string());
            }
        }
    }
    None
}
