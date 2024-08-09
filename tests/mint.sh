#!/usr/bin/env bash

# Number of users to create
NUM_USERS=100

# Function to create a user identity and store its principal ID
create_user_identity() {
  local user_index=$1

  # Create new identity
  dfx identity new user${user_index} --storage-mode=plaintext || true

  # Get the principal ID of the new identity
  local principal_id=$(dfx --identity user${user_index} identity get-principal)

  # Store the principal ID in a temporary file
  echo "${principal_id}" > "principal_${user_index}.txt"
}

# Function to call the mint function using the stored principal ID
mint_user_token() {
  local user_index=$1

  # Retrieve the principal ID from the temporary file
  local principal_id=$(cat "principal_${user_index}.txt")

  # Use the new identity to call the mint function with sample text
  dfx --identity user${user_index} canister call icrc7 mint \
    "(record { owner = record { owner = principal \"${principal_id}\"; subaccount = null }; name = \"Sample Token${user_index}\"; description = null; logo = null })"
}

# Function to delete a user identity (if needed)
delete_user() {
  local user_index=$1
  dfx identity remove user${user_index}
}

# Export the functions to make them available to parallel
export -f create_user_identity
export -f mint_user_token
export -f delete_user

# Measure the time taken to create user identities
start_time_create=$(date +%s)
seq $NUM_USERS | parallel -j10 create_user_identity
end_time_create=$(date +%s)
echo "Time taken to create $NUM_USERS user identities: $((end_time_create - start_time_create)) seconds"

# Measure the time taken to mint tokens
start_time_mint=$(date +%s)
seq $NUM_USERS | parallel -j10 mint_user_token
end_time_mint=$(date +%s)
echo "Time taken to mint tokens for $NUM_USERS users: $((end_time_mint - start_time_mint)) seconds"

# Cleanup temporary files
rm principal_*.txt

# If you need to perform other operations, such as deleting users, you can uncomment the relevant lines below
# start_time_delete=$(date +%s)
# seq $NUM_USERS | parallel -j10 delete_user
# end_time_delete=$(date +%s)
# echo "Time taken to delete $NUM_USERS users: $((end_time_delete - start_time_delete)) seconds"
