// Parity breadcrumbs:
// - packages/bitcoin-knots/test/functional/test_framework
// - packages/bitcoin-knots/test/functional/interface_rpc.py
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::{
    fs,
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
    process::Child,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    pub fn new(label: &str) -> std::io::Result<Self> {
        let root = std::env::temp_dir().join(format!(
            "open-bitcoin-{label}-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn path(&self) -> &Path {
        &self.root
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[derive(Debug)]
pub struct PortReservation {
    listener: TcpListener,
    address: SocketAddr,
}

impl PortReservation {
    pub fn localhost() -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let address = listener.local_addr()?;
        Ok(Self { listener, address })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn into_listener(self) -> TcpListener {
        self.listener
    }
}

#[derive(Debug)]
pub struct ProcessGuard {
    child: Child,
}

impl ProcessGuard {
    pub fn new(child: Child) -> Self {
        Self { child }
    }

    pub fn child_mut(&mut self) -> &mut Child {
        &mut self.child
    }
}

impl Drop for ProcessGuard {
    fn drop(&mut self) {
        match self.child.try_wait() {
            Ok(Some(_status)) => {}
            Ok(None) => {
                let _ = self.child.kill();
                let _ = self.child.wait();
            }
            Err(_error) => {
                let _ = self.child.kill();
                let _ = self.child.wait();
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{PortReservation, Sandbox};

    #[test]
    fn sibling_sandboxes_get_distinct_paths() {
        // Arrange
        let first = Sandbox::new("isolation-test").expect("first sandbox");
        let second = Sandbox::new("isolation-test").expect("second sandbox");

        // Act
        let paths_differ = first.path() != second.path();

        // Assert
        assert!(paths_differ);
        assert!(first.path().exists());
        assert!(second.path().exists());
    }

    #[test]
    fn sibling_port_reservations_get_distinct_addresses() {
        // Arrange
        let first = PortReservation::localhost().expect("first port");
        let second = PortReservation::localhost().expect("second port");

        // Act
        let addresses_differ = first.address() != second.address();

        // Assert
        assert!(addresses_differ);
    }
}
