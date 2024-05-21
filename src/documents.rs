use std::collections::HashMap;
use zbus::zvariant;

#[zbus::proxy(
    interface = "org.freedesktop.portal.Documents",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait DocumentPortal {
    fn add_full<'a>(
        &self,
        o_path_fds: &[zvariant::Fd<'a>],
        flags: u32,
        app_id: &str,
        permissions: &[&str],
    ) -> zbus::Result<(Vec<String>, HashMap<String, zvariant::OwnedValue>)>;
}
