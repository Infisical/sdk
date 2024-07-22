# frozen_string_literal: true

require 'ffi'

module InfisicalSDK
  # C wrapper for Ruby SDK
  module InfisicalLib
    extend FFI::Library

    def self.mac_with_intel?
      `uname -m`.strip == 'x86_64'
    end

    def self.linux_arm64?
      RUBY_PLATFORM.include?('linux') && %w[aarch64 arm64].any? { |arch| RUBY_PLATFORM.include?(arch) }
    end

    ffi_lib case RUBY_PLATFORM
            when /darwin/
              local_file = if mac_with_intel?
                             File.expand_path('macos-x64/libinfisical_c.dylib', __dir__)
                           else
                             File.expand_path('macos-arm64/libinfisical_c.dylib', __dir__)
                           end
              File.exist?(local_file) ? local_file : File.expand_path('../../../../target/debug/libinfisical_c.dylib', __dir__)
            when /linux/
              local_file = if linux_arm64?
                             File.expand_path('linux-arm64/libinfisical_c.so', __dir__)
                           else
                             File.expand_path('linux-x64/libinfisical_c.so', __dir__)
                           end
              File.exist?(local_file) ? local_file : File.expand_path('../../../../target/debug/libinfisical_c.so', __dir__)
            when /mswin|mingw/
              local_file = File.expand_path('windows-x64/infisical_c.dll', __dir__)
              File.exist?(local_file) ? local_file : File.expand_path('../../../../target/debug/infisical_c.dll', __dir__)
            else
              raise "Unsupported platform: #{RUBY_PLATFORM}"
            end

    attach_function :init, [:string], :pointer
    attach_function :run_command, %i[string pointer], :string
    attach_function :free_mem, [:pointer], :void
  end
end