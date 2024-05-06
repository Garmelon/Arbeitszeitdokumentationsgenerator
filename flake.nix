{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, naersk }:
    let
      forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    in
    rec {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          naersk' = pkgs.callPackage naersk { };
          # Create a fontconfig file with liberation_ttf (contains "Liberation Sans")
          fonts = pkgs.makeFontsConf { fontDirectories = with pkgs; [ liberation_ttf ]; };
        in
        rec {
          default = kit-timesheets;
          kit-timesheets = naersk'.buildPackage {
            root = ./.;
          };
          docker = pkgs.dockerTools.buildLayeredImage {
            name = "garmelon/kit-timesheets";
            tag = "latest";

            contents = with pkgs; [
              # Makes debugging the container a bit more pleasant
              busybox
              # Fontconfig is needed so typst will find fonts (renders a blank
              # document otherwise)
              fontconfig
            ];

            config = {
              Entrypoint = [ "${kit-timesheets}/bin/kit_timesheets" ];
              WorkingDir = "/tmp";
              Env = [
                # Fontconfig needs to be babysitted a bit in containers
                "FONTCONFIG_FILE=${fonts}"
                # Useful for read-only containers, as fontconfig will create a
                # cache there
                "HOME=/tmp"
              ];
            };
          };
        }
      );
    };
}
