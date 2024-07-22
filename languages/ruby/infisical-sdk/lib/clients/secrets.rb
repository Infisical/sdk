# frozen_string_literal: true

require 'json'

require_relative '../extended_schemas/schemas'

module InfisicalSDK
  # Manage Infisical secrets.
  class SecretsClient
    def initialize(command_runner)
      @command_runner = command_runner
    end

    # rubocop:disable Metrics/ParameterLists
    # rubocop:disable Metrics/MethodLength
    def get(
      secret_name:,
      project_id:,
      environment:,
      path: nil,
      include_imports: nil,
      type: nil
    )

      response = run_command(get_secret: GetSecretOptions.new(
        secret_name: secret_name,
        project_id: project_id,
        environment: environment,
        path: path,
        include_imports: include_imports,
        get_secret_options_type: type
      ))

      secrets_response = ResponseForGetSecretResponse.from_json!(response).to_dynamic
      error_handler(secrets_response)


      secrets_response['data']['secret']
    end

    def list(
      project_id:,
      environment:,
      path: nil ,
      attach_to_process_env: nil,
      expand_secret_references: nil,
      recursive: nil,
      include_imports: nil
    )
      response = run_command(list_secrets: ListSecretsOptions.new(
        project_id: project_id,
        environment: environment,
        path: path,
        include_imports: include_imports,
        recursive: recursive,
        attach_to_process_env: attach_to_process_env,
        expand_secret_references: expand_secret_references,
      ))

      secrets_response = ResponseForListSecretsResponse.from_json!(response).to_dynamic
      error_handler(secrets_response)

      secrets_response['data']['secrets']
    end

    def update(
      secret_name:,
      secret_value:,
      project_id:,
      environment:,
      path: nil,
      skip_multiline_encoding: nil,
      type: nil
    )
      response = run_command(update_secret: UpdateSecretOptions.new(
        secret_name: secret_name,
        secret_value: secret_value,
        project_id: project_id,
        environment: environment,
        path: path,
        skip_multiline_encoding: skip_multiline_encoding,
        update_secret_options_type: type
      ))

      secrets_response = ResponseForUpdateSecretResponse.from_json!(response).to_dynamic
      error_handler(secrets_response)


      secrets_response['data']['secret']
    end

    def create(
      secret_name:,
      secret_value:,
      project_id:,
      environment:,
      secret_comment: nil,
      path: nil,
      skip_multiline_encoding: nil,
      type: nil

    )

      response = run_command(create_secret: CreateSecretOptions.new(
        secret_name: secret_name,
        secret_value: secret_value,
        secret_comment: secret_comment,
        project_id: project_id,
        environment: environment,
        skip_multiline_encoding: skip_multiline_encoding,
        path: path,
        create_secret_options_type: type
      ))

      secrets_response = ResponseForCreateSecretResponse.from_json!(response).to_dynamic
      error_handler(secrets_response)


      secrets_response['data']['secret']
    end

    def delete(
      secret_name:,
      project_id:,
      environment:,
      path: nil,
      type: nil
    )
      response = run_command(delete_secret: DeleteSecretOptions.new(
        secret_name: secret_name,
        project_id: project_id,
        environment: environment,
        path: path,
        delete_secret_options_type: type
      ))

      secrets_response = ResponseForDeleteSecretResponse.from_json!(response).to_dynamic
      error_handler(secrets_response)

      secrets_response['data']['secret']
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

# rubocop:enable Metrics/ParameterLists
# rubocop:enable Metrics/MethodLength
