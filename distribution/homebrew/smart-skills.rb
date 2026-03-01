class SmartSkills < Formula
  desc "Agent skill management tool - manage and sync AI agent instructions"
  homepage "https://github.com/armedev/smart-skills"
  url "https://github.com/armedev/smart-skills/releases/download/v#{version}/smart-skills-#{version}-x86_64-apple-darwin.tar.gz"
  sha256 "TODO: Update with actual sha256 after first release"
  license "MIT"
  version "0.1.0"

  bottle do
    rebuild 1
    sha256 cellar: :any_skip_relocation, x86_64_linux: "TODO: Update with actual sha256"
  end

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/armedev/smart-skills/releases/download/v#{version}/smart-skills-#{version}-aarch64-apple-darwin.tar.gz"
    else
      url "https://github.com/armedev/smart-skills/releases/download/v#{version}/smart-skills-#{version}-x86_64-apple-darwin.tar.gz"
    end
  end

  on_linux do
    url "https://github.com/armedev/smart-skills/releases/download/v#{version}/smart-skills-#{version}-x86_64-unknown-linux-gnu.tar.gz"
  end

  def install
    bin.install "smart-skills"
  end

  test do
    assert_match "smart-skills", shell_output("#{bin}/smart-skills --help")
  end
end
