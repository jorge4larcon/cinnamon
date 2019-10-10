use std::str::{FromStr};
use std::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6, Ipv6Addr, Ipv4Addr};
use regex::Regex;
use std::fmt;
use std::cmp;

#[derive(Eq, Clone)]
pub struct MacAddress {
    mac: u64
}

impl MacAddress {
    pub fn new(mac: u64) -> Option<MacAddress> {
        match is_mac_u64(mac) {
            true => Some( MacAddress { mac } ),
            false => None
        }
    }

    pub fn new_from_str(mac: &str) -> Option<MacAddress> {
        if let Some(mac) = mac_to_u64(mac) {
            return Some( MacAddress { mac } );
        }
        None
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {        
        write!(f, "{}", u64_to_mac(self.mac, ':').unwrap())
    }    
}

impl cmp::Ord for MacAddress {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.mac.cmp(&other.mac)
    }
}

impl cmp::PartialOrd for MacAddress {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for MacAddress {
    fn eq(&self, other: &Self) -> bool {
        self.mac == other.mac
    }
}

pub struct AddrContainer {
    address: String,
    port: u16,
    iface: String,    
}

impl fmt::Display for AddrContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_v6() {
            write!(f, "{}%{}:{}", self.address, self.iface, self.port)
        } else {
            write!(f, "{}:{}", self.address, self.port)
        }        
    }
}

impl AddrContainer {
    pub fn new(addr: &str) -> Result<AddrContainer, ()> {
        if is_socket_addr(addr) {
            if addr.contains("%") { // IPv6 case
                let addr_scopeport: Vec<&str> = addr.split("%").collect();
                let address = addr_scopeport[0];
                let scope_port: Vec<&str> = addr_scopeport[1].split(":").collect();
                let iface = scope_port[0];
                if let Ok(port) = scope_port[1].parse::<u16>() {
                    return Ok(AddrContainer { address: String::from(address), port, iface: String::from(iface) });
                }
            } else { // IPv4 case
                let addr_port: Vec<&str> = addr.split(":").collect();
                if let Ok(port) = addr_port[1].parse::<u16>() {
                    return Ok(AddrContainer { address: String::from(addr_port[0]), port, iface: String::from("") })
                }
            }
        }
        Err(())
    }

    pub fn to_ip_addr(&self) -> Result<IpAddr, ()> {
        if let Ok(ip) = IpAddr::from_str(self.address.as_str()) {
            return Ok(ip);
        }
        Err(())
    }

    pub fn to_sock_addr(&self) -> SocketAddr {
        if self.is_v4() {
            return SocketAddr::from(self.to_sock_addr4().unwrap());
        } else {
            return SocketAddr::from(self.to_sock_addr6().unwrap());
        }
    }

    pub fn to_sock_addr6(&self) -> Result<SocketAddrV6, ()> {
        if self.is_v6() {
            if let Ok(scp_id) = self.iface.parse::<u32>() {
                if let Ok(mut sock_addr) = SocketAddrV6::from_str(format!("[{}]:{}", self.address, self.port).as_str()) {
                    sock_addr.set_scope_id(scp_id);
                    return Ok(sock_addr);
                }
            }
        }
        Err(())
    }

    pub fn to_sock_addr4(&self) -> Result<SocketAddrV4, ()> {
        if self.is_v4() {
            if let Ok(sock_addr) = SocketAddrV4::from_str(format!("{}:{}", self.address, self.port).as_str()) {
                return Ok(sock_addr);
            }
        }
        Err(())
    }

    pub fn is_v6(&self) -> bool {
        !self.iface.is_empty()
    }

    pub fn is_v4(&self) -> bool {
        self.iface.is_empty()
    }
}

pub fn is_socket_addr(addr: &str) -> bool {
    if addr.is_ascii() {
        if addr.contains("%") { // IPv6 case
            let addr_ifport: Vec<&str> = addr.split("%").collect();
            if addr_ifport.len() == 2 {
                if is_ipv6_addr(addr_ifport[0]) {
                    let scope_port: Vec<&str> = addr_ifport[1].split(":").collect();
                    if scope_port.len() == 2 {
                        if let Ok(port) = scope_port[1].parse::<i32>() {
                            return is_port_number(port);
                        }
                    }
                }
            }
        } else { // IPv4 case
            let addr_port: Vec<&str> = addr.split(":").collect();
            if addr_port.len() == 2 {
                if is_ipv4_addr(addr_port[0]) {
                    if let Ok(port) = addr_port[1].parse::<i32>() {
                        return is_port_number(port);
                    }
                }
            }
        }
    }
    false
}

pub fn is_socket_addr_v4(addr: &str) -> bool {
    match SocketAddrV4::from_str(addr) {
        Ok(_) => true,
        _ => false
    }
}

pub fn is_socket_addr_v6(addr: &str) -> bool {
    match SocketAddrV6::from_str(addr) {
        Ok(_) => true,
        _ => false
    }
}

pub fn is_port_number(p: i32) -> bool {
    p >= 0 && p <= 65535
}

pub fn is_ip_addr(addr: &str) -> bool {
    is_ipv4_addr(addr) || is_ipv6_addr(addr)
}

// TODO: Optimize this function
pub fn is_ipv4_addr(addr: &str) -> bool {
    if let Ok(_) = Ipv4Addr::from_str(addr) {
        return true;
    }
    if addr.is_ascii() {
        let ipv4_re = Regex::new(r"^(?:(?:^|\.)(?:2(?:5[0-5]|[0-4]\d)|1?\d?\d)){4}$").unwrap();
        return ipv4_re.is_match(addr);
    }
    false
}

// TODO: Optimize this function
pub fn is_ipv6_addr(addr: &str) -> bool {
    if let Ok(_) = Ipv6Addr::from_str(addr) {
        return true;
    }
    if addr.is_ascii() {
        let ipv6_re = Regex::new(r"^(?:(?:(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):){6})(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):(?:(?:[0-9a-fA-F]{1,4})))|(?:(?:(?:(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9]))\.){3}(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9])))))))|(?:(?:::(?:(?:(?:[0-9a-fA-F]{1,4})):){5})(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):(?:(?:[0-9a-fA-F]{1,4})))|(?:(?:(?:(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9]))\.){3}(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9])))))))|(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})))?::(?:(?:(?:[0-9a-fA-F]{1,4})):){4})(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):(?:(?:[0-9a-fA-F]{1,4})))|(?:(?:(?:(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9]))\.){3}(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9])))))))|(?:(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):){0,1}(?:(?:[0-9a-fA-F]{1,4})))?::(?:(?:(?:[0-9a-fA-F]{1,4})):){3})(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):(?:(?:[0-9a-fA-F]{1,4})))|(?:(?:(?:(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9]))\.){3}(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9])))))))|(?:(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):){0,2}(?:(?:[0-9a-fA-F]{1,4})))?::(?:(?:(?:[0-9a-fA-F]{1,4})):){2})(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):(?:(?:[0-9a-fA-F]{1,4})))|(?:(?:(?:(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9]))\.){3}(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9])))))))|(?:(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):){0,3}(?:(?:[0-9a-fA-F]{1,4})))?::(?:(?:[0-9a-fA-F]{1,4})):)(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):(?:(?:[0-9a-fA-F]{1,4})))|(?:(?:(?:(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9]))\.){3}(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9])))))))|(?:(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):){0,4}(?:(?:[0-9a-fA-F]{1,4})))?::)(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):(?:(?:[0-9a-fA-F]{1,4})))|(?:(?:(?:(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9]))\.){3}(?:(?:25[0-5]|(?:[1-9]|1[0-9]|2[0-4])?[0-9])))))))|(?:(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):){0,5}(?:(?:[0-9a-fA-F]{1,4})))?::)(?:(?:[0-9a-fA-F]{1,4})))|(?:(?:(?:(?:(?:(?:[0-9a-fA-F]{1,4})):){0,6}(?:(?:[0-9a-fA-F]{1,4})))?::))))$").unwrap();
        return ipv6_re.is_match(addr);
    }
    false
}

pub fn str_to_ipv4addr(addr: &str) -> Option<Ipv4Addr> {
    match Ipv4Addr::from_str(addr) {
        Ok(ipv4) => Some(ipv4),
        _ => None
    }
}

pub fn str_to_ipv6addr(addr: &str) -> Option<Ipv6Addr> {
    match Ipv6Addr::from_str(addr) {
        Ok(ipv6) => Some(ipv6),
        _ => None
    }
}

pub fn parse_ip(addr: &str) -> Result<IpAddr, ()> {
    if let Ok(ip) = IpAddr::from_str(addr) {
        return Ok(ip);
    }
    Err(())
}

// Mac must follow one of the next styles:
// ff:ff:ff:ff:ff:ff
// ff-ff-ff-ff-ff-ff
// ffff.ffff.ffff
pub fn is_mac_str(mac: &str) -> bool {
    if mac.is_ascii() {
        let mac_re = Regex::new(r"^((([a-fA-F0-9][a-fA-F0-9]+[-]){5}|([a-fA-F0-9][a-fA-F0-9]+[:]){5})([a-fA-F0-9][a-fA-F0-9])$)|(^([a-fA-F0-9][a-fA-F0-9][a-fA-F0-9][a-fA-F0-9]+[.]){2}([a-fA-F0-9][a-fA-F0-9][a-fA-F0-9][a-fA-F0-9]))$").unwrap();
        return mac_re.is_match(mac);
    }
    false
}

pub fn mac_to_u64(mac: &str) -> Option<u64> {
    if is_mac_str(mac) {
        let m =  if mac.contains("-") {
            mac.replace("-", "")
        } else if mac.contains(".") {
            mac.replace(".", "")
        } else {
            mac.replace(":", "")
        };
        return Some(u64::from_str_radix(m.as_str(), 16).unwrap());
    }
    return None;
}

// Only 3 separators: `.`, `:` and `-`
pub fn u64_to_mac(mac: u64, sep: char) -> Option<String> {
    if is_mac_u64(mac) {
        let mac = format!("{:012x}", mac);
        match sep {
            '.' => return Some(format!("{}.{}.{}", &mac[0..4], &mac[4..8], &mac[8..12])),
            ':' => return Some(format!("{}:{}:{}:{}:{}:{}", &mac[0..2], &mac[2..4], &mac[4..6], &mac[6..8], &mac[8..10], &mac[10..12])),
            '-' => return Some(format!("{}-{}-{}-{}-{}-{}", &mac[0..2], &mac[2..4], &mac[4..6], &mac[6..8], &mac[8..10], &mac[10..12])),
            _ => return None
        }        
    }
    None
}

// pub fn mac_compare_str(mac: &str, other_mac: &str) -> Result<i8, ()> {
//     let mac = mac_to_u64(mac)?;
//     let other_mac = mac_to_u64(other_mac)?;
//     if mac < other_mac {
//         return Ok(-1);
//     } else if other_mac > mac {
//         return Ok(1);
//     }
//     Ok(0)
// }

pub fn mac_compare_u64(mac: u64, other_mac: u64) -> Result<i8, ()> {
    if is_mac_u64(mac) && is_mac_u64(other_mac) {
        if mac < other_mac {
            return Ok(-1);
        } else if other_mac > mac {
            return Ok(1);
        }
        return Ok(0);
    }
    Err(())
}

// pub fn ipv6_compare_str(ipv6: &str, other_ipv6: &str) -> Result<i8, ()> {
//     let ipv6 = ipv6_to_u128(ipv6)?;
//     let other_ipv6 = ipv6_to_u128(other_ipv6)?;
//     if ipv6 < other_ipv6 {
//         return Ok(-1);
//     } else if ipv6 > other_ipv6 {
//         return Ok(1);
//     }
//     Ok(0)
// }    

pub fn ipv6_compare_u128(ipv6: u128, other_ipv6: u128) -> i8 {
    if ipv6 < other_ipv6 {
        return -1;
    } else if ipv6 > other_ipv6 {
        return 1;
    }
    0
}

pub fn sockaddrv4str_to_sockaddrv4(sockaddrv4: &str) -> Option<SocketAddrV4> {
    if let Ok(sockaddr) = SocketAddrV4::from_str(sockaddrv4) {
        Some(sockaddr)
    } else {
        None
    }
}

pub fn sockaddrv6str_to_ipv6sockaddr(sockaddrv6: &str) -> Option<SocketAddrV6> {
    if let Ok(sockaddr) = SocketAddrV6::from_str(sockaddrv6) {
        Some(sockaddr)
    } else {
        None
    }
}

// pub fn ipv4_compare_str(ipv4: &str, other_ipv4: &str) -> Result<i8, ()> {
//     let ipv4 = ipv4_to_u32(ipv4)?;
//     let other_ipv4 = ipv4_to_u32(other_ipv4)?;
//     if ipv4 < other_ipv4 {
//         return Ok(-1);
//     } else if other_ipv4 > ipv4 {
//         return Ok(1);
//     }
//     Ok(0)
// }

pub fn ipv4_compare_u32(ipv4: u32, other_ipv4: u32) -> i8 {
    if ipv4 < other_ipv4 {
        return -1;
    } else if other_ipv4 > ipv4 {
        return 1;
    }
    0
}

pub fn is_mac_u64(mac: u64) -> bool {
    return mac <= 281474976710655;
}

pub fn ipv6_to_u128(ipv6: &str) -> Option<u128> {
    if is_ipv6_addr(ipv6) {
        let h = Ipv6Addr::from_str(ipv6).unwrap().segments(); // hextets
        // IP Number = (65536^7)*a + (65536^6)*b + (65536^5)*c + (65536^4)*d + (65536^3)*e + (65536^2)*f + 65536*g + h
        // where IP Address = a:b:c:d:e:f:g:h
        let n: u128 = 65536;
        let ip_number = (n.pow(7))*u128::from(h[0]) +
                        (n.pow(6))*u128::from(h[1]) +
                        (n.pow(5))*u128::from(h[2]) +
                        (n.pow(4))*u128::from(h[3]) +
                        (n.pow(3))*u128::from(h[4]) +
                        (n.pow(2))*u128::from(h[5]) +
                        (n)       *u128::from(h[6]) +
                                   u128::from(h[7]);
        return Some(ip_number);
    }
    None
}

pub fn u128_to_ipv6(ipv6: u128) -> Ipv6Addr {
    // let hextets = format!("{:032x}", ipv6);
    // format!(
    //     "{}:{}:{}:{}:{}:{}:{}:{}",
    //     &hextets[0..4],
    //     &hextets[4..8],
    //     &hextets[8..12],
    //     &hextets[12..16],
    //     &hextets[16..20],
    //     &hextets[20..24],
    //     &hextets[24..28],
    //     &hextets[28..])
    Ipv6Addr::from(ipv6)
}

pub fn u32_to_ipv4(ipv4: u32) -> Ipv4Addr {
    // let octets = format!("{:08x}", ipv4);
    // format!(
    //     "{}.{}.{}.{}",
    //     u8::from_str_radix(&octets[0..2], 16).unwrap(),
    //     u8::from_str_radix(&octets[2..4], 16).unwrap(),
    //     u8::from_str_radix(&octets[4..6], 16).unwrap(),
    //     u8::from_str_radix(&octets[8..10], 16).unwrap())
    Ipv4Addr::from(ipv4)
}

// IP Number = 16777216*w + 65536*x + 256*y + z
// where IP Address = w.x.y.z
pub fn ipv4_to_u32(ipv4: &str) -> Option<u32> {
    if is_ipv4_addr(ipv4) {
        let o = Ipv4Addr::from_str(ipv4).unwrap().octets();
        let ip_number: u32 = 16777216*u32::from(o[0]) +
                             65536   *u32::from(o[1]) +
                             256     *u32::from(o[2]) +
                                      u32::from(o[3]);
        return Some(ip_number);
    }
    None
}

pub fn ipv4addr_to_u32(ipv4addr: &Ipv4Addr) -> u32 {
    let o = ipv4addr.octets();
    let ip_number: u32 = 16777216*u32::from(o[0]) +
                            65536   *u32::from(o[1]) +
                            256     *u32::from(o[2]) +
                                    u32::from(o[3]);
    return ip_number;
}

pub fn ipv6addr_to_u128(ipv6addr: &Ipv6Addr) -> u128 {
    let h = ipv6addr.segments(); // hextets
    // IP Number = (65536^7)*a + (65536^6)*b + (65536^5)*c + (65536^4)*d + (65536^3)*e + (65536^2)*f + 65536*g + h
    // where IP Address = a:b:c:d:e:f:g:h
    let n: u128 = 65536;
    let ip_number = (n.pow(7))*u128::from(h[0]) +
                    (n.pow(6))*u128::from(h[1]) +
                    (n.pow(5))*u128::from(h[2]) +
                    (n.pow(4))*u128::from(h[3]) +
                    (n.pow(3))*u128::from(h[4]) +
                    (n.pow(2))*u128::from(h[5]) +
                    (n)       *u128::from(h[6]) +
                                u128::from(h[7]);
    return ip_number;
}

pub fn ipv6addr_str_to_u128(ipaddr: &IpAddr) -> Option<u128> {    
    if let IpAddr::V6(ipv6) = ipaddr {
        return Some(ipv6addr_to_u128(ipv6));
    }
    None
}

pub fn ipv4addr_str_to_u32(ipaddr: &IpAddr) -> Option<u32> {    
    if let IpAddr::V4(ipv4) = ipaddr {
        return Some(ipv4addr_to_u32(ipv4));
    }
    None
}
