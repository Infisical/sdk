
# Note, this for for testing purposes. This should be changed to [require 'infisical-sdk'] in your code.
require_relative '../infisical-sdk/lib/infisical-sdk'

# Update these to your own values
project_id = 'f1617cbc-be46-4466-89de-ec8767afeaab'
env_slug = 'dev'

# Update these to your own values
universal_auth_client_id = 'YOUR_IDENTITY_UNIVERSAL_AUTH_CLIENT_ID'
universal_auth_client_secret = 'YOUR_IDENTITY_UNIVERSAL_AUTH_CLIENT_SECRET'


test_secret_name = 'TEST_SECRET' # Note, this secret needs to already exist in the project/environment that you're testing against.

test_create_secret_name = 'A_NEW_SECRET' # This secret will be created
test_create_secret_value = 'SOME-API-KEY' # The value of the new secret that will be created

test_update_secret_new_value = 'NEW VALUE' # The secret created above will be updated to this value

# 1. Create the Infisical client
infisical = InfisicalSDK::InfisicalClient.new

# 2. Authenticate the Infisical Client
infisical.auth.universal_auth(client_id: universal_auth_client_id, client_secret: universal_auth_client_secret)

# 3. Use the Infisical client after authentication
secrets = infisical.secrets.list(project_id: project_id, environment: env_slug) # ... Takes more parameters, but only project ID and env slug is required.
secrets.each_with_index { |secret, index| puts "#{index}: #{secret['secretKey']}: #{secret['secretValue']}" }

# 4. Get a single secret
puts "Getting secret with name '#{test_secret_name}'"
single_test_secret = infisical.secrets.get(
  secret_name: test_secret_name,
  project_id: project_id,
  environment: env_slug
)
puts "Test Secret: #{single_test_secret}\n\n"


# 5. Create a secret
puts 'Creating new secret...'
new_secret = infisical.secrets.create(
  secret_name: test_create_secret_name,
  secret_value: test_create_secret_value,
  project_id: project_id,
  environment: env_slug
)
puts "New Secret: #{new_secret}\n\n"


# 6. Update the newly created secret's value to be "NEW VALUE"
puts 'Updating secret...'
updated_secret = infisical.secrets.update(
  secret_name: test_create_secret_name,
  secret_value: test_update_secret_new_value,
  project_id: project_id,
  environment: env_slug
)
puts "Updated Secret: #{updated_secret}\n\n"


# 7. Finally, delete the secret entirely
puts 'Deleting newly created secret...'
deleted_secret = infisical.secrets.delete(
  secret_name: test_create_secret_name,
  project_id: project_id,
  environment: env_slug
)
puts "Deleted Secret: #{deleted_secret}\n\n"


### Encryption tests:
plaintext_data = 'Hello World'
key = infisical.encryption.create_symmetric_key

encrypted_data = infisical.encryption.encrypt_symmetric(data: plaintext_data, key: key)

decrypted_data = infisical.encryption.decrypt_symmetric(
  ciphertext: encrypted_data['ciphertext'],
  iv: encrypted_data['iv'],
  tag: encrypted_data['tag'],
  key: key
)

puts "Plaintext: #{plaintext_data}\n"
puts "Encryption key: #{key}\n"
puts "Encrypted ciphertext: #{encrypted_data['ciphertext']}\n"
puts "Decrypted text: #{decrypted_data}\n"
