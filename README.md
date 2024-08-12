# CertificationNFT Canister

This project implements a simple certification NFT system on the Internet Computer (IC) using Rust and the Candid interface. The contract manages the creation, transfer, and management of NFTs representing certifications. The owner of the contract can grant and revoke manager roles to other principals. Managers can mint NFTs, transfer them, and set base URIs for token metadata. The contract also supports batch minting and transferring of NFTs.

## Features

- **Minting NFTs:** Allows managers to mint individual or batches of NFTs.
- **Token Transfer:** Managers can transfer NFTs between accounts.
- **Metadata Management:** Set and retrieve base URIs and token URIs for NFTs.
- **Role Management:** The owner can grant and revoke manager roles to other principals.

## Prerequisites

- **DFX SDK:** Make sure you have the DFX SDK installed. You can install it by following the instructions [here](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove/).
- **Rust:** Ensure you have Rust installed. You can install it from [here](https://www.rust-lang.org/tools/install).

## Setup and Deployment

1. **Clone the Repository:**

   git clone https://github.com/your-repo-url
   cd your-repo-folder


Deploy the Canister:

You can deploy the canister using the provided deploy.sh script. First, make sure the script has the necessary execution permissions:

chmod +x deploy.sh

Then, run the script:

./deploy.sh

This will deploy the canister to the local IC environment.