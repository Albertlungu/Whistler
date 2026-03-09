class Whistler < Formula
  desc "Lightweight code editor built with Rust and iced"
  homepage "https://github.com/Albertlungu/Whistler"
  url "https://github.com/Albertlungu/Whistler/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "7795e8aac155d430db450b6ddd87afbdacd966fbc1ebd046959f0e4e2dbd8f51"
  license "MIT"

  depends_on "rust" => :build

  def install
    # The Rust crate is in the nested whistler/ directory.
    system "cargo", "install", *std_cargo_args(path: "whistler")

    # Provide a conventional lowercase executable name.
    bin.install_symlink bin/"Whistler" => "whistler"
  end

  test do
    assert_predicate bin/"Whistler", :exist?
    assert_predicate bin/"whistler", :exist?
  end
end
