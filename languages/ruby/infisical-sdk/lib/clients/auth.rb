# frozen_string_literal: true

require 'json'
require_relative '../extended_schemas/schemas'

module InfisicalSDK
  # Manage Infisical secrets.
  class AuthClient
    def initialize(command_runner)
      @command_runner = command_runner
    end

    def universal_auth(client_id:, client_secret:)
      response = run_command(universal_auth_login: UniversalAuthMethod.new(
        client_id: client_id,
        client_secret: client_secret
      ))

      handle_auth_response(response)
    end

    def kubernetes_auth(identity_id:, service_account_token_path:)

      response = run_command(kubernetes_auth_login: KubernetesAuthMethod.new(
        identity_id: identity_id,
        service_account_token_path: service_account_token_path
      ))

      handle_auth_response(response)
    end

    def azure_auth(identity_id:)

      response = run_command(azure_auth_login: AzureAuthMethod.new(
        identity_id: identity_id
      ))

      handle_auth_response(response)
    end

    def gcp_id_token_auth(identity_id:)

      response = run_command(gcp_id_token_auth_login: GCPIDTokenAuthMethod.new(
        identity_id: identity_id
      ))

      handle_auth_response(response)
    end

    def gcp_iam_auth(identity_id:, service_account_key_file_path:)

      response = run_command(gcp_iam_auth_login: GCPIamAuthMethod.new(
        identity_id: identity_id,
        service_account_key_file_path: service_account_key_file_path
      ))

      handle_auth_response(response)
    end

    def aws_iam_auth(identity_id:)

      response = run_command(aws_iam_auth_login: AWSIamAuthMethod.new(
        identity_id: identity_id
      ))

      handle_auth_response(response)
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

    def handle_auth_response(response)
      auth_response = ResponseForAccessTokenSuccessResponse.from_json!(response).to_dynamic
      error_handler(auth_response)

      auth_response['data']
    end

    def run_command(command)
      response = @command_runner.run(InfisicalCommands.new(command))
      raise InfisicalError, 'Error getting response' if response.nil?

      response
    end
  end
end

