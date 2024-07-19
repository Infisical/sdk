# frozen_string_literal: true

module InfisicalSDK
  class InfisicalError < StandardError
    def initialize(message = 'Failed to get get response')
      super(message)
    end
  end
end