pub mod detection;
pub mod error;
pub mod page;
pub mod port;
pub mod proxy_pool;
pub mod request;
pub mod validation;
pub mod workflow;

pub use error::{ScrapeError, ScrapeResult};
pub use page::{PageCookie, ScrapedPage};
pub use port::{BrowserSession, CdpEndpoint, PageScraper};
pub use proxy_pool::{ProxyPoolConfig, RotationStrategy};
pub use request::{ProxyConfig, ScrapeRequest};
pub use workflow::WorkflowStep;
