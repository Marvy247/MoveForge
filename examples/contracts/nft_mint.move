/// Concurrent NFT Minting Example for Movement
/// Demonstrates parallel execution and conflict detection
module moveforge_nft::concurrent_mint {
    use std::signer;
    use std::string::{Self, String};
    use aptos_framework::timestamp;
    use aptos_framework::account;
    use aptos_framework::event;

    /// Error codes
    const E_NOT_INITIALIZED: u64 = 1;
    const E_ALREADY_MINTED: u64 = 2;
    const E_MINT_LIMIT_REACHED: u64 = 3;
    const E_MINT_NOT_STARTED: u64 = 4;
    const E_INSUFFICIENT_PAYMENT: u64 = 5;

    /// NFT Collection resource
    struct NFTCollection has key {
        name: String,
        total_minted: u64,
        max_supply: u64,
        mint_price: u64,
        mint_start_time: u64,
    }

    /// Individual NFT resource
    struct NFT has key, store {
        id: u64,
        name: String,
        uri: String,
        minted_at: u64,
        owner: address,
    }

    /// Minting record to prevent double mints
    struct MintRecord has key {
        nfts_owned: u64,
    }

    /// Mint event
    struct MintEvent has drop, store {
        minter: address,
        token_id: u64,
        timestamp: u64,
    }

    /// Collection events
    struct CollectionEvents has key {
        mint_events: event::EventHandle<MintEvent>,
    }

    /// Initialize the NFT collection
    public entry fun initialize_collection(
        admin: &signer,
        name: vector<u8>,
        max_supply: u64,
        mint_price: u64,
        mint_start_time: u64,
    ) {
        let admin_addr = signer::address_of(admin);
        
        let collection = NFTCollection {
            name: string::utf8(name),
            total_minted: 0,
            max_supply,
            mint_price,
            mint_start_time,
        };
        
        move_to(admin, collection);
        
        let events = CollectionEvents {
            mint_events: account::new_event_handle<MintEvent>(admin),
        };
        move_to(admin, events);
    }

    /// Mint a new NFT - can be called concurrently by multiple users
    public entry fun mint_nft(
        minter: &signer,
        collection_addr: address,
    ) acquires NFTCollection, MintRecord, CollectionEvents {
        let minter_addr = signer::address_of(minter);
        
        // Check collection exists
        assert!(exists<NFTCollection>(collection_addr), E_NOT_INITIALIZED);
        
        let collection = borrow_global_mut<NFTCollection>(collection_addr);
        
        // Check mint has started
        let now = timestamp::now_seconds();
        assert!(now >= collection.mint_start_time, E_MINT_NOT_STARTED);
        
        // Check supply limit
        assert!(collection.total_minted < collection.max_supply, E_MINT_LIMIT_REACHED);
        
        // Increment total minted (this is where conflicts can occur in parallel execution)
        let token_id = collection.total_minted;
        collection.total_minted = collection.total_minted + 1;
        
        // Create the NFT
        let nft = NFT {
            id: token_id,
            name: collection.name,
            uri: generate_token_uri(token_id),
            minted_at: now,
            owner: minter_addr,
        };
        
        // Store NFT with the minter
        move_to(minter, nft);
        
        // Update or create mint record
        if (!exists<MintRecord>(minter_addr)) {
            let record = MintRecord {
                nfts_owned: 1,
            };
            move_to(minter, record);
        } else {
            let record = borrow_global_mut<MintRecord>(minter_addr);
            record.nfts_owned = record.nfts_owned + 1;
        };
        
        // Emit mint event
        let events = borrow_global_mut<CollectionEvents>(collection_addr);
        event::emit_event(
            &mut events.mint_events,
            MintEvent {
                minter: minter_addr,
                token_id,
                timestamp: now,
            },
        );
    }

    /// Stress test: mint multiple NFTs in rapid succession
    /// This function demonstrates where conflicts occur in parallel execution
    public entry fun batch_mint(
        minter: &signer,
        collection_addr: address,
        count: u64,
    ) acquires NFTCollection, MintRecord, CollectionEvents {
        let i = 0;
        while (i < count) {
            mint_nft(minter, collection_addr);
            i = i + 1;
        }
    }

    /// Generate token URI based on token ID
    fun generate_token_uri(token_id: u64): String {
        let base_uri = string::utf8(b"ipfs://moveforge-nft/");
        let id_str = u64_to_string(token_id);
        string::append(&mut base_uri, id_str);
        string::append_utf8(&mut base_uri, b".json");
        base_uri
    }

    /// Convert u64 to string (simplified)
    fun u64_to_string(num: u64): String {
        if (num == 0) {
            return string::utf8(b"0")
        };
        
        let result = vector::empty<u8>();
        let temp = num;
        
        while (temp > 0) {
            let digit = ((temp % 10) as u8) + 48; // ASCII '0' is 48
            vector::push_back(&mut result, digit);
            temp = temp / 10;
        };
        
        vector::reverse(&mut result);
        string::utf8(result)
    }

    /// Get collection info
    public fun get_collection_info(collection_addr: address): (u64, u64, u64) acquires NFTCollection {
        let collection = borrow_global<NFTCollection>(collection_addr);
        (collection.total_minted, collection.max_supply, collection.mint_price)
    }

    /// Get user's NFT count
    public fun get_user_nft_count(user_addr: address): u64 acquires MintRecord {
        if (!exists<MintRecord>(user_addr)) {
            return 0
        };
        let record = borrow_global<MintRecord>(user_addr);
        record.nfts_owned
    }

    #[test_only]
    use aptos_framework::account;

    #[test(admin = @moveforge_nft, user1 = @0x123, user2 = @0x456)]
    public fun test_concurrent_minting(admin: &signer, user1: &signer, user2: &signer) acquires NFTCollection, MintRecord, CollectionEvents {
        // Setup
        account::create_account_for_test(signer::address_of(admin));
        account::create_account_for_test(signer::address_of(user1));
        account::create_account_for_test(signer::address_of(user2));
        timestamp::set_time_has_started_for_testing(&account::create_signer_for_test(@0x1));
        
        // Initialize collection
        initialize_collection(admin, b"MoveForge NFT", 1000, 100, 0);
        
        // Simulate concurrent minting
        mint_nft(user1, signer::address_of(admin));
        mint_nft(user2, signer::address_of(admin));
        
        // Verify state
        let (minted, max, _price) = get_collection_info(signer::address_of(admin));
        assert!(minted == 2, 0);
        assert!(max == 1000, 1);
        
        assert!(get_user_nft_count(signer::address_of(user1)) == 1, 2);
        assert!(get_user_nft_count(signer::address_of(user2)) == 1, 3);
    }
}
