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
- **GNU Parallel:** Required for running parallel tasks in scripts. Install GNU Parallel by following the instructions for your operating system:
  - **Ubuntu/Debian:** `sudo apt-get install parallel`
  - **CentOS/RHEL:** First, install EPEL: `sudo yum install epel-release`, then `sudo yum install parallel`
  - **macOS:** Using Homebrew: `brew install parallel`

## Setup and Deployment

### Clone the Repository

```bash
git clone https://github.com/ICP-hub/genun-certification.git
cd genun-certification
```

### Install Dependencies

Before deploying the canister, you need to install necessary tools and set up the environment:

1. **Install Required Tools:**

   ```bash
   cargo install ic-wasm
   cargo install candid-extractor
   ```

   These tools are used for building and extracting interface definitions from your Rust code.

2. **Add the WebAssembly Target:**

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

   This step adds support for compiling Rust code to WebAssembly, which is necessary for deploying code to the Internet Computer.

### Prepare the Local IC Environment

Before you can deploy the canister, you need to prepare and start the local IC environment:

1. **Load Environment Variables:**

   If you are using macOS, you might need to source the environment variables for `dfx`:

   ```bash
   source "$HOME/Library/Application Support/org.dfinity.dfx/env"
   ```

2. **Start the Local IC Replica:**

   To interact with your canisters during development, you need to start a local IC replica:

   ```bash
   dfx start --clean
   ```

   This command starts a clean local replica of the IC, ensuring there are no remnants from previous projects.

### Deploy the Canister

You can deploy the canister using the provided `deploy.sh` script. First, make sure the script has the necessary execution permissions:

```bash
chmod +x deploy.sh
```

Then, run the script:

```bash
./deploy.sh
```

This will deploy the canister to the local IC environment.

## Managing and Cleaning Up Canisters

After testing and development, you might need to clean up the canisters deployed in your local IC environment. Here are the steps to stop and delete canisters:

### Stopping All Canisters

To halt all canister operations safely, use the following command:

```bash
dfx canister stop --all
```

### Deleting Canisters

If you need to remove specific canisters from your local environment, you can delete them by using the delete command. For this project, you may need to delete the following canisters:

```bash
dfx canister delete genun_backend
dfx canister delete icrc7
```

These commands will remove the specified canisters, ensuring that they no longer occupy resources or retain any data.

## Additional Information

Ensure all commands are executed in the root directory of the project. If you encounter any issues, check that all prerequisites are installed and that the `deploy.sh` script is correctly configured according to your system and project settings.
