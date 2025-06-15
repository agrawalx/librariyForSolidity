// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title IModExpContract
 * @dev This interface is for documentation and clarity.
 * Due to the custom selector in the Rust contract (0x01),
 * we must use a low-level .call() to interact with it, not a
 * high-level call through this interface.
 */
interface IModExpContract {
    function modexp(uint64 base, uint64 exp, uint64 modulus) external pure returns (uint64);
}

/**
 * @title CallRustModExp
 * @dev This contract calls a Rust smart contract that performs modular exponentiation.
 * It manually constructs the calldata to match the Rust contract's expectations,
 * including a custom function selector.
 */
contract CallRustModExp {

    /**
     * @notice Calls the modexp function in the Rust contract.
     * @param rustContractAddress The on-chain address of the deployed Rust contract.
     * @param base The base of the exponentiation.
     * @param exp The exponent.
     * @param modulus The modulus.
     * @return result The result of (base ^ exp) % modulus.
     */
    function callModExp(
        address rustContractAddress,
        uint64 base,
        uint64 exp,
        uint64 modulus
    ) public view returns (uint64) {
        // The Rust contract expects a specific 4-byte selector: 0x00000001 (or 0x01).
        bytes4 selector = bytes4(0x00000001);

        // The Rust contract reads three 32-byte words for its arguments.
        // abi.encode() correctly pads our uint64 values to 32 bytes.
        // We concatenate the selector and the encoded arguments to build the calldata.
        bytes memory payload = abi.encodePacked(
            selector,
            abi.encode(base),
            abi.encode(exp),
            abi.encode(modulus)
        );

        // Use a low-level staticcall since the function is pure/view.
        (bool success, bytes memory returnData) = rustContractAddress.staticcall(payload);

        // Revert if the call to the Rust contract failed.
        require(success, "Low-level call to Rust contract failed");

        // The Rust contract returns a 32-byte value where the u64 result
        // is in the last 8 bytes. abi.decode() can parse this directly into a uint256.
        uint256 result256 = abi.decode(returnData, (uint256));

        // Safely cast the result to uint64 and return it.
        return uint64(result256);
    }

}

