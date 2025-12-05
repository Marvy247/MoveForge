// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title HybridBridge
 * @notice Example Solidity contract that bridges with Move modules on Movement
 * @dev Demonstrates hybrid Move-EVM interactions for MoveForge testing
 */
contract HybridBridge {
    // Events
    event MessageSentToMove(address indexed sender, bytes32 indexed messageId, bytes data);
    event MessageReceivedFromMove(bytes32 indexed messageId, address indexed recipient, bytes data);
    event BridgeTransfer(address indexed from, address indexed to, uint256 amount, string direction);

    // State
    mapping(bytes32 => bool) public processedMessages;
    mapping(address => uint256) public balances;
    address public moveModuleAddress;
    uint256 public messageNonce;

    // Errors
    error MessageAlreadyProcessed();
    error InsufficientBalance();
    error InvalidMoveAddress();
    error ZeroAmount();

    constructor(address _moveModuleAddress) {
        require(_moveModuleAddress != address(0), "Invalid Move module address");
        moveModuleAddress = _moveModuleAddress;
    }

    /**
     * @notice Send a message from EVM to Move module
     * @param data The message data to send
     */
    function sendToMove(bytes calldata data) external returns (bytes32) {
        messageNonce++;
        bytes32 messageId = keccak256(abi.encodePacked(msg.sender, messageNonce, data));
        
        emit MessageSentToMove(msg.sender, messageId, data);
        
        // In production, this would trigger Movement's EVM-to-Move bridge
        // For MoveForge testing, we simulate the cross-VM call
        
        return messageId;
    }

    /**
     * @notice Receive and process a message from Move module
     * @param messageId Unique identifier for the message
     * @param recipient The recipient address on EVM side
     * @param data The message data from Move
     */
    function receiveFromMove(
        bytes32 messageId,
        address recipient,
        bytes calldata data
    ) external {
        if (processedMessages[messageId]) revert MessageAlreadyProcessed();
        
        processedMessages[messageId] = true;
        
        emit MessageReceivedFromMove(messageId, recipient, data);
        
        // Process the message (example: mint tokens, update state, etc.)
        _processMessage(recipient, data);
    }

    /**
     * @notice Bridge tokens from EVM to Move
     * @param amount Amount to bridge
     */
    function bridgeToMove(uint256 amount) external {
        if (amount == 0) revert ZeroAmount();
        if (balances[msg.sender] < amount) revert InsufficientBalance();
        
        balances[msg.sender] -= amount;
        
        bytes memory bridgeData = abi.encode(msg.sender, amount);
        sendToMove(bridgeData);
        
        emit BridgeTransfer(msg.sender, moveModuleAddress, amount, "to_move");
    }

    /**
     * @notice Bridge tokens from Move to EVM
     * @param recipient Recipient on EVM side
     * @param amount Amount to bridge
     */
    function bridgeFromMove(address recipient, uint256 amount) external {
        // In production, this would verify Move module signature
        // For testing, we trust the caller
        
        balances[recipient] += amount;
        
        emit BridgeTransfer(moveModuleAddress, recipient, amount, "from_move");
    }

    /**
     * @notice Deposit tokens into the bridge
     */
    function deposit() external payable {
        balances[msg.sender] += msg.value;
    }

    /**
     * @notice Withdraw tokens from the bridge
     * @param amount Amount to withdraw
     */
    function withdraw(uint256 amount) external {
        if (balances[msg.sender] < amount) revert InsufficientBalance();
        
        balances[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }

    /**
     * @notice Process message data received from Move
     * @param recipient The recipient address
     * @param data The message data
     */
    function _processMessage(address recipient, bytes calldata data) internal {
        // Example processing: decode and mint tokens
        (uint256 amount) = abi.decode(data, (uint256));
        balances[recipient] += amount;
    }

    /**
     * @notice Get bridge status for an address
     * @param user The user address
     */
    function getBridgeStatus(address user) external view returns (
        uint256 balance,
        uint256 currentNonce
    ) {
        return (balances[user], messageNonce);
    }

    /**
     * @notice Check if message has been processed
     * @param messageId The message identifier
     */
    function isMessageProcessed(bytes32 messageId) external view returns (bool) {
        return processedMessages[messageId];
    }
}
