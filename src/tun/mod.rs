mod error;
pub use error::Error;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::NativeTun;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::NativeTun;

use async_trait::async_trait;

#[async_trait]
pub trait Tun: Send + Sync + Clone {
    fn name(&self) -> &str;
    fn mtu(&self) -> Result<u16, Error>;
    fn set_mtu(&self, mtu: u16) -> Result<(), Error>;
    async fn recv(&self) -> Result<Vec<u8>, Error>;
    async fn send(&self, buf: &[u8]) -> Result<(), Error>;
}
