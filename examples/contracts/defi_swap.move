/// Simple DEX swap module for Movement
/// Demonstrates parallel execution and resource management
module paraforge_examples::defi_swap {
    use std::signer;
    use aptos_framework::coin::{Self, Coin};
    use aptos_framework::timestamp;

    /// Error codes
    const E_INSUFFICIENT_BALANCE: u64 = 1;
    const E_POOL_NOT_INITIALIZED: u64 = 2;
    const E_SLIPPAGE_EXCEEDED: u64 = 3;
    const E_ZERO_AMOUNT: u64 = 4;

    /// Liquidity pool resource
    struct LiquidityPool<phantom CoinA, phantom CoinB> has key {
        reserve_a: Coin<CoinA>,
        reserve_b: Coin<CoinB>,
        total_shares: u64,
        last_update: u64,
    }

    /// User liquidity position
    struct LiquidityPosition<phantom CoinA, phantom CoinB> has key {
        shares: u64,
    }

    /// Initialize a new liquidity pool
    public entry fun initialize_pool<CoinA, CoinB>(
        admin: &signer,
        initial_a: u64,
        initial_b: u64,
    ) {
        let admin_addr = signer::address_of(admin);
        
        // Create pool with initial liquidity
        let pool = LiquidityPool<CoinA, CoinB> {
            reserve_a: coin::withdraw<CoinA>(admin, initial_a),
            reserve_b: coin::withdraw<CoinB>(admin, initial_b),
            total_shares: initial_a + initial_b,
            last_update: timestamp::now_seconds(),
        };
        
        move_to(admin, pool);
        
        // Give admin initial shares
        let position = LiquidityPosition<CoinA, CoinB> {
            shares: initial_a + initial_b,
        };
        move_to(admin, position);
    }

    /// Swap CoinA for CoinB
    public entry fun swap_a_to_b<CoinA, CoinB>(
        user: &signer,
        amount_in: u64,
        min_amount_out: u64,
    ) acquires LiquidityPool {
        assert!(amount_in > 0, E_ZERO_AMOUNT);
        
        let pool_addr = @paraforge_examples;
        assert!(exists<LiquidityPool<CoinA, CoinB>>(pool_addr), E_POOL_NOT_INITIALIZED);
        
        let pool = borrow_global_mut<LiquidityPool<CoinA, CoinB>>(pool_addr);
        
        // Calculate output using constant product formula: x * y = k
        let reserve_a = coin::value(&pool.reserve_a);
        let reserve_b = coin::value(&pool.reserve_b);
        
        let amount_out = calculate_swap_output(amount_in, reserve_a, reserve_b);
        assert!(amount_out >= min_amount_out, E_SLIPPAGE_EXCEEDED);
        
        // Execute swap
        let coins_in = coin::withdraw<CoinA>(user, amount_in);
        coin::merge(&mut pool.reserve_a, coins_in);
        
        let coins_out = coin::extract(&mut pool.reserve_b, amount_out);
        coin::deposit(signer::address_of(user), coins_out);
        
        pool.last_update = timestamp::now_seconds();
    }

    /// Swap CoinB for CoinA
    public entry fun swap_b_to_a<CoinA, CoinB>(
        user: &signer,
        amount_in: u64,
        min_amount_out: u64,
    ) acquires LiquidityPool {
        assert!(amount_in > 0, E_ZERO_AMOUNT);
        
        let pool_addr = @paraforge_examples;
        assert!(exists<LiquidityPool<CoinA, CoinB>>(pool_addr), E_POOL_NOT_INITIALIZED);
        
        let pool = borrow_global_mut<LiquidityPool<CoinA, CoinB>>(pool_addr);
        
        let reserve_a = coin::value(&pool.reserve_a);
        let reserve_b = coin::value(&pool.reserve_b);
        
        let amount_out = calculate_swap_output(amount_in, reserve_b, reserve_a);
        assert!(amount_out >= min_amount_out, E_SLIPPAGE_EXCEEDED);
        
        let coins_in = coin::withdraw<CoinB>(user, amount_in);
        coin::merge(&mut pool.reserve_b, coins_in);
        
        let coins_out = coin::extract(&mut pool.reserve_a, amount_out);
        coin::deposit(signer::address_of(user), coins_out);
        
        pool.last_update = timestamp::now_seconds();
    }

    /// Calculate swap output using constant product formula
    /// Formula: amount_out = (amount_in * reserve_out) / (reserve_in + amount_in)
    fun calculate_swap_output(
        amount_in: u64,
        reserve_in: u64,
        reserve_out: u64,
    ): u64 {
        let numerator = (amount_in as u128) * (reserve_out as u128);
        let denominator = (reserve_in as u128) + (amount_in as u128);
        ((numerator / denominator) as u64)
    }

    /// Get pool reserves
    public fun get_reserves<CoinA, CoinB>(): (u64, u64) acquires LiquidityPool {
        let pool = borrow_global<LiquidityPool<CoinA, CoinB>>(@paraforge_examples);
        (coin::value(&pool.reserve_a), coin::value(&pool.reserve_b))
    }

    #[test_only]
    use aptos_framework::account;

    #[test(admin = @paraforge_examples)]
    public fun test_initialize_pool(admin: &signer) {
        account::create_account_for_test(signer::address_of(admin));
        timestamp::set_time_has_started_for_testing(&account::create_signer_for_test(@0x1));
        
        // Test pool initialization would go here
        // In real implementation, would use test coins
    }
}
