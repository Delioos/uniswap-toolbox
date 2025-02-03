# Usage
This is a homemade rust library to play with uniswap liquidity pools.

I plan to split the lib in two separate crates:

1. `uniswap-math`: A mathematics library that provides:
   - Q number calculations and conversions
   - Pricing calculations with tick math
   - Fee calculations and arithmetic
   - Liquidity math helpers
   - Square root price calculations

2. `uniswap-client`: (Future crate) Will handle on-chain operations by wrapping alloy with chain-specific contract addresses for:
   - Pool creation and management
   - Pool state queries
   - Position management

## Getting Started with uniswap-math

To use the math library in your project:

1. Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
uniswap-math = "0.1.0"
```

# Personal notes
- might add a uniswap-backtest crate later to facilitate my backtesting strategies 
