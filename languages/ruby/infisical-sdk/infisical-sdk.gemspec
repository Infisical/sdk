# frozen_string_literal: true

require_relative 'lib/version'

Gem::Specification.new do |spec|
  spec.name = 'infisical-sdk'
  spec.version = InfisicalSDK::VERSION
  spec.authors = ['Infisical Inc.']
  spec.email = ['team@infisical.com']

  spec.summary = 'Ruby SDK for interacting with the Infisical platform.'
  spec.description = 'The official Infisical Ruby SDK.'
  spec.homepage = 'https://infisical.com'
  spec.required_ruby_version = '>= 2.7'

  spec.metadata['homepage_uri'] = spec.homepage
  spec.metadata['source_code_uri'] = 'https://github.com/infisical/sdk'
  spec.metadata['changelog_uri'] = 'https://infisical.com/docs/changelog/overview'

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) || f.start_with?(*%w[bin/ test/ spec/ features/ .git Gemfile])
    end
  end

  spec.files += Dir.glob('lib/linux-x64/**/*')
  spec.files += Dir.glob('lib/linux-arm64/**/*')
  spec.files += Dir.glob('lib/macos-x64/**/*')
  spec.files += Dir.glob('lib/windows-x64/**/*')
  spec.files += Dir.glob('lib/macos-arm64/**/*')
  spec.files += Dir.glob('lib/schemas.rb')

  spec.bindir = 'exe'
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"
  spec.add_dependency 'dry-struct', '~> 1.6'
  spec.add_dependency 'dry-types', '~> 1.7'
  spec.add_dependency 'ffi', '~> 1.15'
  spec.add_dependency 'json', '~> 2.6'
  spec.add_dependency 'rake', '~> 13.0'
  spec.add_dependency 'rubocop', '~> 1.21'

end
