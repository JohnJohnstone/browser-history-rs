{
  description = "browser-history";

  inputs = {
	  nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
	  rust-overlay.url = "github:oxalica/rust-overlay";
	  flake-utils.url  = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
	flake-utils.lib.eachDefaultSystem (system:
	let
		overlays = [ (import rust-overlay) ];
		pkgs = import nixpkgs {
		  inherit system overlays;
		};
		rustbin = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
	in
	with pkgs;
	{
		devShells.default = mkShell {
		  buildInputs = [
        rustbin
        clippy
        rust-analyzer
        fd
        just
		  ];
		  shellHook = ''
        alias j=just
	      alias ls=exa
			  alias find=fd
		  '';
	  };
	}
  );
}
