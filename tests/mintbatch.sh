#!/usr/bin/env bash

# Number of users to create
NUM_USERS=100
BATCH_SIZE=10

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

# Function to batch mint tokens using the stored principal IDs
mint_batch_tokens() {
  local batch_index=$1
  local start_index=$((batch_index * BATCH_SIZE))
  local end_index=$((start_index + BATCH_SIZE - 1))

  local owners=()
  local names=()
  local descriptions=()
  local logos=()

  for user_index in $(seq $start_index $end_index); do
    if [ -f "principal_${user_index}.txt" ]; then
      local principal_id=$(cat "principal_${user_index}.txt")
      owners+=("(record { owner = principal \"${principal_id}\"; subaccount = null })")
      names+=("\"Sample Token${user_index}\"")
      descriptions+=("null")
      logos+=("null")
    fi
  done

  local owners_str=$(IFS=, ; echo "${owners[*]}")
  local names_str=$(IFS=, ; echo "${names[*]}")
  local descriptions_str=$(IFS=, ; echo "${descriptions[*]}")
  local logos_str=$(IFS=, ; echo "${logos[*]}")

  # Call the mint_batch function
  dfx canister call icrc7 mint_batch \
    "(record { owners = vec { ${owners_str} }; names = vec { ${names_str} }; descriptions = vec { ${descriptions_str} }; logos = vec { ${logos_str} } })"
}

# Function to delete a user identity (if needed)
delete_user() {
  local user_index=$1
  dfx identity remove user${user_index}
}

# Export the functions to make them available to parallel
export -f create_user_identity
export -f mint_batch_tokens
export -f delete_user

# Measure the time taken to create user identities
start_time_create=$(date +%s)
seq $NUM_USERS | parallel -j10 create_user_identity
end_time_create=$(date +%s)
echo "Time taken to create $NUM_USERS user identities: $((end_time_create - start_time_create)) seconds"

# Measure the time taken to mint tokens in batches
start_time_mint=$(date +%s)
total_batches=$((NUM_USERS / BATCH_SIZE))
seq 0 $((total_batches - 1)) | parallel -j1 mint_batch_tokens
end_time_mint=$(date +%s)
echo "Time taken to mint tokens for $NUM_USERS users in batches: $((end_time_mint - start_time_mint)) seconds"

# Cleanup temporary files
rm principal_*.txt

# If you need to perform other operations, such as deleting users, you can uncomment the relevant lines below
# start_time_delete=$(date +%s)
# seq $NUM_USERS | parallel -j10 delete_user
# end_time_delete=$(date +%s)
# echo "Time taken to delete $NUM_USERS users: $((end_time_delete - start_time_delete)) seconds"
