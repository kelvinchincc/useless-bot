#! /usr/bin/env ruby
# frozen_string_literal: true

require 'date'

is_mac = RbConfig::CONFIG['host_os'] =~ /darwin|mac os/
image = 'useless-bot'
snapshot_version = Time.now.strftime('%Y%m%dT%H%M%S')
platform = is_mac ? 'linux/arm64' : 'linux/amd64'
output_dir = 'target/docker'

docker_build_command = 'docker buildx build'
platform_option = "--platform #{platform}"
tag_option = "-t #{image}:#{snapshot_version}"
output_options = "--output type=docker,dest=#{output_dir}/#{image}-#{snapshot_version}.tar"
docker_build_command = "#{docker_build_command} #{platform_option} #{tag_option} #{output_options} ."

if is_mac
  puts "Detected macOS. Building for platform #{platform}..."
else
  puts "Detected non-macOS. Building for platform #{platform}..."
end

puts "Building Docker image with tag #{image}:#{snapshot_version}..."
exec docker_build_command
