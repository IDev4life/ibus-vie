use tracing::debug;

/// IBus Service interface implementation.
///
/// IBus daemon calls Destroy on this interface when it wants to tear down
/// an engine instance. We handle it as a no-op since the object server
/// cleanup happens automatically.
pub struct IbusService;

#[zbus::interface(name = "org.freedesktop.IBus.Service")]
impl IbusService {
    fn destroy(&self) {
        debug!("IBus.Service.Destroy called");
    }
}
