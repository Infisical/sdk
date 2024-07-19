# frozen_string_literal: true

require 'json'
require 'dry-types'

require_relative 'schemas'
require_relative 'infisical_lib'
require_relative 'infisical_error'

module InfisicalSDK
  class InfisicalSettings
    attr_accessor :api_url, :identity_url

    def initialize(api_url, identity_url)
      # if api_url.nil? || identity_url.nil?
      #   raise ArgumentError, "api_url and identity_url cannot be nil"
      # end

      @api_url = api_url
      @identity_url = identity_url
    end
  end

  class InfisicalClient
    attr_reader :infisical

    def initialize(infisical_settings)
      client_settings = ClientSettings.new()

      @infisical = InfisicalLib
      @handle = @infisical.init(client_settings.to_dynamic.compact.to_json)

      # @command_runner = CommandRunner.new(@infisical, @handle)
      # @project_client = ProjectsClient.new(@command_runner)
      # @secrets_client = SecretsClient.new(@command_runner)
    end

    def free_mem
      @infisical.free_mem(@handle)
    end
  end
end
