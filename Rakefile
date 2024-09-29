# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("fast-polylines.gemspec")

RbSys::ExtensionTask.new("fast_polylines", GEMSPEC) do |ext|
  ext.lib_dir = "lib/fast_polylines"
end

task default: %i[compile spec rubocop]
