use super::*;

pub(super) const fn resource_unit(kind: ResourceBoundKind) -> ResourceBoundUnit {
    match kind {
        ResourceBoundKind::Disk
        | ResourceBoundKind::Cache
        | ResourceBoundKind::Log
        | ResourceBoundKind::SupportBundle => ResourceBoundUnit::Bytes,
        ResourceBoundKind::File => ResourceBoundUnit::Files,
        ResourceBoundKind::Metric => ResourceBoundUnit::Items,
        ResourceBoundKind::Peer => ResourceBoundUnit::Peers,
        ResourceBoundKind::Queue | ResourceBoundKind::InFlight => ResourceBoundUnit::Requests,
    }
}
