# frozen_string_literal: true

require 'json'
require 'dry-types'

require_relative 'schemas'
require_relative 'extended_schemas/schemas'
require_relative 'infisical_lib'
require_relative 'infisical_error'
require_relative 'command_runner'
require_relative 'clients/secrets'
require_relative 'clients/auth'
require_relative 'clients/cryptography'

module InfisicalSDK
  class InfisicalClient
    attr_reader :infisical, :command_runner, :secrets, :auth, :cryptography

    def initialize(site_url = "https://app.infisical.com", cache_ttl = 300)
      settings = ClientSettings.new(
        # We preset these values or we'll get type validation errors (thanks Quicktype!)
        access_token: nil,
        client_secret: nil,
        client_id: nil,
        auth: nil,
        ssl_certificate_path: nil,
        user_agent: 'infisical-ruby-sdk',
        cache_ttl: cache_ttl,
        site_url: site_url
      )


      @infisical = InfisicalLib
      @handle = @infisical.init(settings.to_dynamic.compact.to_json)
      @command_runner = CommandRunner.new(@infisical, @handle)
      @secrets = SecretsClient.new(@command_runner)
      @auth = AuthClient.new(@command_runner)
      @cryptography = CryptographyClient.new(@command_runner)
    end

    def free_mem
      @infisical.free_mem(@handle)
    end
  end
end
