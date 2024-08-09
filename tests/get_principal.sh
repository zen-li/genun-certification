# Number of users to create
NUM_USERS=50

# Path to Project1 directory
PROJECT1_DIR="/home/harman/genun_certification/tests"

# Create users and store their principals in Project1
create_users_and_store_principals() {
  cd $PROJECT1_DIR

  for i in $(seq 0 $((NUM_USERS-1))); do
    
    # Switch to new identity
    dfx identity use user${i}
    
    # Get principal of the current identity
    principal=$(dfx identity get-principal)
    

  done
}

# Execute the function
create_users_and_store_principals