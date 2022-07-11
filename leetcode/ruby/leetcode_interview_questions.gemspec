# frozen_string_literal: true

require_relative 'lib/leetcode_interview_questions/version'

Gem::Specification.new do |spec|
  spec.name = 'leetcode_interview_questions'
  spec.version = LeetcodeInterviewQuestions::VERSION
  spec.authors = ['Ryan Cammer']
  spec.email = ['ryancammer@gmail.com']

  spec.summary = 'Answers to leetcode interview questions'
  spec.homepage = 'https://github.com/ryancammer/interview_questions'
  spec.license = 'MIT'
  spec.required_ruby_version = '>= 3.1.2'

  spec.metadata['homepage_uri'] = spec.homepage
  spec.metadata['source_code_uri'] = 'https://github.com/ryancammer/interview_questions'
  spec.metadata['changelog_uri'] = 'https://github.com/ryancammer/interview_questions/CHANGELOG.md'

  spec.files = Dir.chdir(File.expand_path(__dir__)) do
    `git ls-files -z`.split('\x0').reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end
  end

  spec.bindir = 'exe'
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']

  spec.add_development_dependency 'rspec', '~> 3.11'
end
