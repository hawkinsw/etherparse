use crate::{ArpHardwareId, EtherType, LinuxNonstandardEtherType};

/// Represents the "protcol type" field in a Linux Cooked Capture v1 packet. It
/// is represented as an enum due to the meaning of the inner value depending 
/// on the associated arp_hardware_id field. 
///
/// You can convert pairs of ArpHardwareId and its associated u16 value with `
/// LinuxSllProtocolType::try_from()`, an Err(_) is returned if the relation is
/// not defined or known.
///
/// ```
/// use etherparse::LinuxNonstandardEtherType;
///
/// // Convert to LinuxNonstandardEtherType using the from & into trait
/// let link_type: LinuxNonstandardEtherType = 0x0001.try_into().unwrap();
/// assert_eq!(LinuxNonstandardEtherType::N802_3, link_type);
///
/// // convert to u16 using the from & into trait
/// let num: u16 = LinuxNonstandardEtherType::N802_3.try_into().unwrap();
/// assert_eq!(0x0001, num);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinuxSllProtocolType {
    /// The protocol type should be ignored
    Ignored(u16),
    /// Netlink protocol type of the encapsulated payload
    NetlinkProtocolType(u16),
    /// Generic Routing Encapsulation protocol type for the encapsulated payload
    GenericRoutingEncapsulationProtocolType(u16),
    /// EtherType of the encapsulated payload
    EtherType(EtherType),
    /// Non-standard ether types of the encapsulated payload
    LinuxNonstandardEtherType(LinuxNonstandardEtherType),
}

impl TryFrom<(ArpHardwareId, u16)> for LinuxSllProtocolType {
    type Error = ();

    fn try_from((arp_hardware_id, protocol_type): (ArpHardwareId, u16)) -> Result<Self, Self::Error> {
        match arp_hardware_id {
            ArpHardwareId::NETLINK => Ok(LinuxSllProtocolType::NetlinkProtocolType(protocol_type)),
            ArpHardwareId::IPGRE => Ok(LinuxSllProtocolType::GenericRoutingEncapsulationProtocolType(protocol_type)),
            ArpHardwareId::IEEE80211_RADIOTAP => Ok(LinuxSllProtocolType::Ignored(protocol_type)),
            ArpHardwareId::FRAD => Ok(LinuxSllProtocolType::Ignored(protocol_type)),
            ArpHardwareId::ETHER => match LinuxNonstandardEtherType::try_from(protocol_type) {
                Ok(v) => Ok(LinuxSllProtocolType::LinuxNonstandardEtherType(v)),
                Err(_) => Ok(LinuxSllProtocolType::EtherType(EtherType(protocol_type)))
            },
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_from_pair_arp_hardware_id_u16() {
        assert_eq!(LinuxSllProtocolType::try_from((ArpHardwareId::NETLINK, 123)), Ok(LinuxSllProtocolType::NetlinkProtocolType(123)));
        assert_eq!(LinuxSllProtocolType::try_from((ArpHardwareId::IPGRE, 123)), Ok(LinuxSllProtocolType::GenericRoutingEncapsulationProtocolType(123)));
        assert_eq!(LinuxSllProtocolType::try_from((ArpHardwareId::IEEE80211_RADIOTAP, 123)), Ok(LinuxSllProtocolType::Ignored(123)));
        assert_eq!(LinuxSllProtocolType::try_from((ArpHardwareId::FRAD, 123)), Ok(LinuxSllProtocolType::Ignored(123)));
        assert_eq!(LinuxSllProtocolType::try_from((ArpHardwareId::ETHER, u16::from(LinuxNonstandardEtherType::N802_3))), Ok(LinuxSllProtocolType::LinuxNonstandardEtherType(LinuxNonstandardEtherType::N802_3)));
        assert_eq!(LinuxSllProtocolType::try_from((ArpHardwareId::ETHER, u16::from(EtherType::IPV4))), Ok(LinuxSllProtocolType::EtherType(EtherType::IPV4)));
    }
}
