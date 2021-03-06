use chromiumoxide_cdp::cdp::browser_protocol::browser::BrowserContextId;

/// BrowserContexts provide a way to operate multiple independent browser
/// sessions.
#[derive(Debug)]
pub struct BrowserContext {
    id: BrowserContextId,
}

impl BrowserContext {}
