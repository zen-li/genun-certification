#!/usr/bin/env bash

# Number of users to create
NUM_USERS=100

# Function to create a user identity and update details
create_user() {
  local user_index=$1
  local email="user${user_index}@example.com"
  local firstName="First${user_index}"
  local lastName="Last${user_index}"

  # Create new identity
  dfx identity new user${user_index} --storage-mode=plaintext || true
    
  # Call the canister function to update user details
    dfx --identity user${user_index} canister call icrc7 mint "(\"$email\", \"$firstName\", \"$lastName\")"

 }



# Function to delete a user identity


# Export the functions to make them available to parallel
export -f create_user
export -f get_cartitems
export -f delete_user

# Measure the time taken to create users
start_time_create=$(date +%s)
seq $NUM_USERS | parallel -j10 create_user
end_time_create=$(date +%s)

# Optionally, you can measure the time taken to perform other operations like get_cartitems
# start_time_get=$(date +%s)
# seq $NUM_USERS | parallel -j10 get_cartitems
# end_time_get=$(date +%s)

echo "Time taken to create $NUM_USERS users: $((end_time_create - start_time_create)) seconds"
# echo "Time taken to get cart items: $((end_time_get - start_time_get)) seconds"

