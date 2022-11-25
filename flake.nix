{
  outputs = { self, nixpkgs, ... }: {
    packages.x86_64-linux.default = let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in pkgs.rustPlatform.buildRustPackage {
      pname = "test";
      version = "0";

      src = self;
      cargoLock = {
        lockFile = ./Cargo.lock;
        outputHashes = let
          hash = "sha256-QBjUJQKxV3KNNF0Kj3oLKW5SQb1MWdciHqk+QIaLils=";
        in {
          "hass-mqtt-discovery-0.1.0" = hash;
          "hass-mqtt-discovery-macros-0.0.0" = hash;
        };
      };
    };
  };
}
