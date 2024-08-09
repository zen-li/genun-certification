// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;


// npm packages:
//   erc721a@4.2.3 https://www.npmjs.com/package/erc721a
//   solmate@6.7.0 https://www.npmjs.com/package/solmate
//   @openzeppelin/contracts@4.9.2 https://www.npmjs.com/package/@openzeppelin/contracts

/**
 * @title CertificationNFT
 * @dev A simple ERC721A implementation for creating NFTs for certifications.
 * ERC721A is an extension of ERC721 that allows for the minting of multiple tokens at once,
 * and cut down on gas fees.
 * The owner of the contract can grant and revoke manager roles to other addresses.
 * Managers can mint NFTs and set the base URI for the token URI.
 * The base URI is prepended to the token ID to form the token URI.
 * The token URI is used to retrieve the metadata for the token.
 * NFT owner can transfer the NFT to another address.
 */

import "erc721a/contracts/ERC721A.sol";
import "solmate/src/tokens/ERC721.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/utils/Context.sol";


contract CertificationNFT is ERC721A, Context {
    /**
     * @dev Emitted when the base URI is set.
     * @param oldBaseURI The old base URI.
     * @param newBaseURI The new base URI.
     */
    event SetBaseURI(
        string oldBaseURI,
        string newBaseURI
    );

    /**
     * @dev Emitted when a batch of NFTs are minted.
     * @param to The address that received the NFTs.
     * @param amount The number of NFTs minted.
     * @param startTokenId The token ID of the first NFT minted.
     * @param endTokenId The token ID of the last NFT minted.
     */
    event MintBatch (
        address to,
        uint256 amount,
        uint256 startTokenId,
        uint256 endTokenId
    );

    using Strings for uint256;
    string private _baseTokenURI;
    address private _owner;
    mapping(address => bool) private isManager;

    /**
     * @dev Constructor that sets the params of the contract.
     * The owner of the contract is granted manager role.
     * @param __name The name of the NFT.
     * @param __symbol The symbol of the NFT.
     * @param __baseURI The base URI for the token URI.
     */
    constructor(
        string memory __name,
        string memory __symbol,
        string memory __baseURI
    ) ERC721A(__name, __symbol) {
        _baseTokenURI = __baseURI;
        _owner = _msgSender();
        isManager[_msgSender()] = true;
    }

    /**
     * After testing, changing it to 1 will reduce the gas fee for minting,
     * but will increase the fee for transferring, resulting in an overall
     * increase in cost.
     * (Testing with a single mint quantity of 1000)
     */
    /**
     * @dev Returns the token ID of the first NFT that will be minted.
     */
    function _startTokenId() internal override view virtual returns (uint256) {
        return 1;
    }

    /**
     * @dev Grants manager role to an address.
     * @param manager The address to grant manager role to.
     */
    function grantManager(address manager) public {
        require(_owner == _msgSender(), "UnauthorizedGrantManager");
        isManager[manager] = true;
    }

    /**
     * @dev Revokes manager role from an address.
     * @param manager The address to revoke manager role from.
     */
    function revokeManager(address manager) public {
        require(_owner == _msgSender(), "UnauthorizedRevokeManager");
        require(_owner != manager, "CannotRevokeOwnerFromManager");
        require(isManager[manager], "InvalidRevokeManager");
        isManager[manager] = false;
    }


    /**
     * @dev Overrides the _beforeTokenTransfers function in ERC721A.
     * Only managers are allowed to transfer NFTs.
     * _param from The address that the NFT is being transferred from.
     * _param to The address that the NFT is being transferred to.
     * _param startTokenId The token ID of the first NFT being transferred.
     * _param quantity The number of NFTs being transferred.
     */
    function _beforeTokenTransfers(
        address, // from,
        address, // to,
        uint256, // startTokenId,
        uint256  // quantity
    ) internal virtual override {
        require(isManager[_msgSender()], "UnauthorizedTransfer");
    }

    /**
     * @dev Overrides the isApprovedForAll function in ERC721.
     * Managers are approved for all transfers.
     * @param owner The owner of the NFT.
     * @param operator The address that is being approved for all transfers.
     * @return bool True if the operator is approved for all transfers, false otherwise.
     */
    function isApprovedForAll(address owner, address operator) public view override returns (bool) {
        if (isManager[operator]) {
            return true;
        }

        return super.isApprovedForAll(owner, operator);
    }

    /**
     * @dev Overrides the approve function in ERC721.
     * Prohibit the use of the approve function.
     */
    function approve(address, uint256) public override payable {
        require(false, "UnauthorizedApprove");
    }

    /**
     * @dev Overrides the setApprovalForAll function in ERC721.
     * Prohibit the use of the setApprovalForAll function.
     */
    function setApprovalForAll(address, bool) public override pure {
        require(false, "UnauthorizedSetApprovalForAll");
    }

    /**
     * After testing, it was found that _initializeOwnershipAt did not reduce
     * the cost of the initial transfer of NFT, but significantly increased
     * the cost of minting, resulting in a doubling of the cost compared to
     * when _initializeOwnershipAt was not used.
     * (Testing with a single mint quantity of 1000)
     */
    /**
     * @dev Mints an NFT and assigns it to an address.
     * Only managers are allowed to mint NFTs.
     * @param to The address that will receive the NFT.
     */
    function mint(address to) public {
        require(isManager[_msgSender()], "UnauthorizedMint");
        _mint(to, 1);
    }

    /**
     * @dev Mints a batch of NFTs and assigns them to an address.
     * Only managers are allowed to mint NFTs.
     * @param to The address that will receive the NFTs.
     * @param amount The number of NFTs to mint.
     * The token ID of the first NFT minted and the token ID of the lastNFT
     * minted are emitted in the MintBatch event.
     */
    function mintBatch(address to, uint256 amount) public {
        require(isManager[_msgSender()], "UnauthorizedMint");
        uint256 startTokenId = _nextTokenId();
        _mint(to, amount);
        emit MintBatch(to, amount, startTokenId, startTokenId + amount);
    }

    /**
     * @dev Returns the base URI for the token URI.
     */
    function baseURI() public view returns (string memory) {
        return _baseTokenURI;
    }

    /**
     * @dev Sets the base URI for the token URI.
     * Only managers are allowed to set the base URI.
     * @param uri The new base URI.
     * The old base URI and the new base URI are emitted in the SetBaseURI event.
     */
    function setBaseURI(string calldata uri) external {
        require(isManager[_msgSender()], "UnauthorizedSetBaseURI");
        _baseTokenURI = uri;
        emit SetBaseURI(
            _baseTokenURI,
            uri
        );
    }

    /**
     * @dev Returns the token URI for a token ID.
     * @param tokenId The token ID to retrieve the token URI for.
     * @return string The token URI for the token ID.
     * If the base URI is not set, an empty string is returned.
     */
    function tokenURI(uint256 tokenId) public view virtual override returns (string memory) {
        require(_exists(tokenId), "NonExistentToken");

        return bytes(_baseTokenURI).length > 0 ? string(abi.encodePacked(_baseTokenURI, tokenId.toString(), ".json")) : "";
    }
}