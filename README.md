# Usage
This is a homemade rust library to play with uniswap liquidity pools.

I plan to split the lib in two separate crates.

A maths lib to facilitate operations like Q numbers calculs, pricing math with ticks, fees calculator.

Then their might be a second crate to perform operations onchain like open a pool, get pool infos etc, this will be made by wrapping alloy with needed contracts address by chains.

