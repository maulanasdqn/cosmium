pub mod detection;
pub mod error;
pub mod page;
pub mod port;
pub mod request;
pub mod validation;
pub mod workflow;

pub use error::{ScrapeError, ScrapeResult};
pub use page::{PageCookie, ScrapedPage};
pub use port::{BrowserSession, CdpEndpoint, PageScraper};
pub use request::{ProxyConfig, ScrapeRequest};
pub use workflow::WorkflowStep;
