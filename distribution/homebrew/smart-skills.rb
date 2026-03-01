class SmartSkills < Formula
  desc "Agent skill management tool - manage and sync AI agent instructions"
  homepage "https://github.com/armedev/smart-skills"
  license "MIT"

  # Version
  VERSION = "0.1.1"

  # SHA256 checksums
  SHA256_AARCH64_DARWIN = "8be927a4687fbf81daad90df8dbacc70ba126e19fab694acab29a320e080bafc"
  SHA256_X86_64_DARWIN = "09895adbee2c7a968b9af549a0d291d7d0173c73c82db29fcaad8e1b76e5ede8"
  SHA256_X86_64_LINUX = "43057924a5c78dcab0b605cf549710029c05bac1ef597b8a432434df30ce6ec4"

  version VERSION

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/armedev/smart-skills/releases/download/v#{VERSION}/smart-skills-v#{VERSION}-aarch64-apple-darwin.tar.gz"
      sha256 SHA256_AARCH64_DARWIN
    else
      url "https://github.com/armedev/smart-skills/releases/download/v#{VERSION}/smart-skills-v#{VERSION}-x86_64-apple-darwin.tar.gz"
      sha256 SHA256_X86_64_DARWIN
    end
  end

  on_linux do
    url "https://github.com/armedev/smart-skills/releases/download/v#{VERSION}/smart-skills-v#{VERSION}-x86_64-unknown-linux-gnu.tar.gz"
    sha256 SHA256_X86_64_LINUX
  end

  def install
    bin.install "smart-skills"
  end

  test do
    assert_match "smart-skills", shell_output("#{bin}/smart-skills --help")
  end
end
