{ rustPlatform
, glibc
, targetPlatform
, lib
, buildGNUStatic ? false
, patchInterpreter ? null
}:
rustPlatform.buildRustPackage ({
  name = "server";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
} // (
  if buildGNUStatic then {
    buildInputs = [ glibc.static ];
    RUSTFLAGS = [ "-C" "target-feature=+crt-static" ];
  }
  else
    lib.optionalAttrs (patchInterpreter != null) {
      postBuild = ''
        # touch additional-output
        ls -la target
        patchelf --set-interpreter ${patchInterpreter} target/armv7-unknown-linux-gnueabihf/release/server
      '';
    }
))
