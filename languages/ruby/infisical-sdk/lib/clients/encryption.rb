# frozen_string_literal: true

# rubocop:disable Metrics/MethodLength
# rubocop:disable Naming/MethodParameterName



require 'json'
require_relative '../extended_schemas/schemas'


module InfisicalSDK
  # Perform encryption
  class EncryptionClient
    def initialize(command_runner)
      @command_runner = command_runner
    end

    def encrypt_symmetric(data:, key:)
      response = run_command(encrypt_symmetric: EncryptSymmetricOptions.new(
        key: key,
        plaintext: data,
      ))

      encrypt_response = ResponseForEncryptSymmetricResponse.from_json!(response).to_dynamic
      error_handler(encrypt_response)


      encrypt_response['data']
    end

    def decrypt_symmetric(
      ciphertext:,
      iv:,
      tag:,
      key:
    )

      response = run_command(decrypt_symmetric: DecryptSymmetricOptions.new(
        ciphertext: ciphertext,
        iv: iv,
        tag: tag,
        key: key
      ))

      decrypt_response = ResponseForDecryptSymmetricResponse.from_json!(response).to_dynamic
      error_handler(decrypt_response)

      decrypt_response['data']['decrypted']
    end

    def create_symmetric_key
      response = run_command(create_symmetric_key: ArbitraryOptions.new(
        data: ''
      ))

      key_response = ResponseForCreateSymmetricKeyResponse.from_json!(response).to_dynamic
      error_handler(key_response)

      key_response['data']['key']
    end

    private

    def error_handler(response)

      # If the response is successful, we return without raising errors.
      if response.key?('success') && response['success'] == true && response.key?('data')
        return
      end

      if response['errorMessage']
        raise InfisicalError, response['errorMessage'] if response.key?('errorMessage')
      else
        raise InfisicalError, 'Error while getting response'
      end
    end

    def run_command(command)
      response = @command_runner.run(InfisicalCommands.new(command))
      raise InfisicalError, 'Error getting response' if response.nil?

      response
    end
  end
end

# rubocop:enable Metrics/MethodLength
# rubocop:enable Naming/MethodParameterName

