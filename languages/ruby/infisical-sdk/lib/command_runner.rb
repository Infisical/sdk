# frozen_string_literal: true

module InfisicalSDK
  # Run base SDK commands
  class CommandRunner
    def initialize(infisical_sdk, handle)
      @infisical_sdk = infisical_sdk
      @handle = handle
    end

    # @param [Dry-Struct] cmd
    def run(cmd)
      @infisical_sdk.run_command(cmd.to_json, @handle)
    end
  end
end
