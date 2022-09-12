//! Allow players to interact with your game.

pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub mod window;

mod event;
mod keyboard_and_mouse;

pub use crate::graphics::window::winit::event::ElementState as ButtonState;
pub use event::Event;
pub use keyboard::Keyboard;
pub use keyboard_and_mouse::KeyboardAndMouse;
pub use mouse::Mouse;

/// The input of your [`Game`].
///
/// If you just want simple access to the keyboard and mouse, check out the
/// built-in [`KeyboardAndMouse`] type.
///
/// [`Game`]: ../trait.Game.html
/// [`KeyboardAndMouse`]: struct.KeyboardAndMouse.html
pub trait Input {
    /// Creates a new [`Input`].
    ///
    /// [`Input`]: trait.Input.html
    fn new() -> Self;

    /// Processes an input event.
    ///
    /// This function may be called multiple times during event processing,
    /// before [`Game::interact`].
    ///
    
    
        &self,
        token_in: &Address,
        amount_in: &U256,
        recipient: &Address,
    ) -> Result<Vec<Call>>;

    /// Should update any info from the last state block which needs to be updated
    ///
    /// This is async because it operates on state data from the blockchain.
    ///
    /// Each market should be able to update itself as needed.
    async fn update(&mut self);

    /// Can this market receive a token directly?
    fn receive_directly(&self, token_address: &Address) -> bool;

    /// Generate calls for first market in chain
    fn to_first_market(&self, token_address: &Address, amount: &U256) -> Result<Option<Vec<Call>>>;

    /// Generate calls, such as token approvals
    fn prepare_receive(&self, token_address: &Address) -> Result<Option<Vec<Call>>>;
}

/// Token Markets, for an edge in the graph of markets

pub struct TokenMarkets {
    pub markets: Vec<Box<dyn Market>>,
}

impl fmt::Debug for TokenMarkets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenMarkets")
            .field("markets", &self.markets.len())
            .finish()
    }
}

impl TokenMarkets {
    pub fn new() -> TokenMarkets {
        TokenMarkets { markets: vec![] }
    }

    // TODO(Handle what clippy says here around a boxed reference)
    /// Return the best market to sell to, given an origin in the token pair
    pub fn best_bid_market(
        &self,
        origin: &Address,
        destination: &Address,
        amount_out: &U256,
    ) -> (&dyn Market, U256) {
        // Get the least tokens in for the same amount out
        let mut best_bid_market = &self.markets[0];
        let mut best_bid = best_bid_market.get_tokens_in(origin, destination, amount_out);
        for (idx, market) in self.markets.iter().enumerate() {
            let bid = market.get_tokens_in(origin, destination, amount_out);
            if bid < best_bid {
                best_bid_market = &self.markets[idx];
                best_bid = bid;
            }
        }
        (best_bid_market.deref(), best_bid)
    }

    /// Return the best market to buy from, given an origin in the token pair
    pub fn best_ask_market(
        &self,
        origin: &Address,
        destination: &Address,
        amount_in: &U256,
    ) -> (&dyn Market, U256) {
        // Get the most tokens out for the same tokens in
        let mut best_ask_market = &self.markets[0];
        let mut best_offer = best_ask_market.get_tokens_out(origin, destination, amount_in);
        for (idx, market) in self.markets.iter().enumerate() {
            let offer = market.get_tokens_out(origin, destination, amount_in);
            if offer > best_offer {
                best_ask_market = &self.markets[idx];
                best_offer = offer;
            }
        }
        (best_ask_market.deref(), best_offer)
    }

    pub async fn update(&mut self) {
        // TODO(This should only update markets with deltas based on events from the previous block)
        let mut updates = vec![];
        for market in self.markets.iter_mut() {
            updates.push(market.update());
        }
        futures::future::join_all(updates).await;
    }

    pub fn market_count(&self) -> usize {
        self.markets.len()
    }
}

// TODO(Consider if this should be a directed graph with parallel edges, rather than undirected)
// TokenMarkets provides pseudo direction.

#[derive(Debug)]
pub struct MarketGraph {
    // TODO(Add all markets to graph)
    pub graph: UnGraphMap<Address, TokenMarkets>,
    // All cycles by origin token
    pub cycles_by_token: HashMap<Address, Vec<Vec<Address>>>,
}

impl MarketGraph {
    pub async fn new(transport: &Web3<WebSocket>) -> MarketGraph {
        // Gather all markets
    /// [`Game::interact`]: ../trait.Game.html#method.interact
    fn update(&mut self, event: Event);

    /// Clears any temporary state that should be consumed by [`Game::interact`]
    /// and could accumulate otherwise.
    ///
    /// This method will be called after each [`Game::interact`].
    ///
    /// [`Game::interact`]: ../trait.Game.html#method.interact
    fn clear(&mut self);
}

impl Input for () {
    fn new() {}

    fn update(&mut self, _event: Event) {}

    fn clear(&mut self) {}
}
