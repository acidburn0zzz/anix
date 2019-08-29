use super::config::ConfigDescriptor;
use super::device::DeviceDescriptor;
use super::endpoint::EndpointDescriptor;
use super::interface::InterfaceDescriptor;
use super::setup::Setup;

#[repr(u8)]
pub enum DescriptorKind {
    None,
    Device,
    Configuration,
    String,
    Interface,
    Endpoint,
    DeviceQualifier,
    OtherSpeedConfiguration,
    InterfacePower,
    OnTheGo,
} 
