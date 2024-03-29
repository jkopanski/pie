# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.

args@{
  release ? true,
  rootFeatures ? [
    "tree-sitter-pie/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  hostPlatformCpu ? null,
  hostPlatformFeatures ? [],
  target ? null,
  codegenOpts ? null,
  profileOpts ? null,
  cargoUnstableFlags ? null,
  rustcLinkFlags ? null,
  rustcBuildFlags ? null,
  mkRustCrate,
  rustLib,
  lib,
  workspaceSrc,
  ignoreLockHash,
}:
let
  nixifiedLockHash = "358054975d164063f4b9bf9fe66e5773956a972b5ed6c18263d0d5254f449364";
  workspaceSrc = if args.workspaceSrc == null then ./. else args.workspaceSrc;
  currentLockHash = builtins.hashFile "sha256" (workspaceSrc + /Cargo.lock);
  lockHashIgnored = if ignoreLockHash
                  then builtins.trace "Ignoring lock hash" ignoreLockHash
                  else ignoreLockHash;
in if !lockHashIgnored && (nixifiedLockHash != currentLockHash) then
  throw ("Cargo.nix ${nixifiedLockHash} is out of sync with Cargo.lock ${currentLockHash}")
else let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName ({ profile, profileName }: mkRustCrate ({ inherit release profile hostPlatformCpu hostPlatformFeatures target profileOpts codegenOpts cargoUnstableFlags rustcLinkFlags rustcBuildFlags; } // (f profileName)));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  cargo2nixVersion = "0.11.0";
  workspace = {
    tree-sitter-pie = rustPackages.unknown.tree-sitter-pie."0.0.1";
  };
  "registry+https://github.com/rust-lang/crates.io-index".aho-corasick."1.1.2" = overridableMkRustCrate (profileName: rec {
    name = "aho-corasick";
    version = "1.1.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b2969dcb958b36655471fc61f7e416fa76033bdd4bfed0678d8fee1e2d07a1f0"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "perf-literal" ]
      [ "std" ]
    ];
    dependencies = {
      memchr = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".memchr."2.7.1" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".cc."1.0.83" = overridableMkRustCrate (profileName: rec {
    name = "cc";
    version = "1.0.83";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "f1174fb0b6ec23863f8b971027804a42614e347eafb0a95bf0b12cdae21fc4d0"; };
    dependencies = {
      ${ if hostPlatform.isUnix then "libc" else null } = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.152" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".libc."0.2.152" = overridableMkRustCrate (profileName: rec {
    name = "libc";
    version = "0.2.152";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "13e3bf6590cbc649f4d1a3eefc9d5d6eb746f5200ffb04e5e142700b8faa56e7"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".memchr."2.7.1" = overridableMkRustCrate (profileName: rec {
    name = "memchr";
    version = "2.7.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "523dc4f511e55ab87b694dc30d0f820d60906ef06413f93d4d7a1385599cc149"; };
    features = builtins.concatLists [
      [ "alloc" ]
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".regex."1.10.2" = overridableMkRustCrate (profileName: rec {
    name = "regex";
    version = "1.10.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "380b951a9c5e80ddfd6136919eef32310721aa4aacd4889a8d39124b026ab343"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "perf" ]
      [ "perf-backtrack" ]
      [ "perf-cache" ]
      [ "perf-dfa" ]
      [ "perf-inline" ]
      [ "perf-literal" ]
      [ "perf-onepass" ]
      [ "std" ]
      [ "unicode" ]
      [ "unicode-age" ]
      [ "unicode-bool" ]
      [ "unicode-case" ]
      [ "unicode-gencat" ]
      [ "unicode-perl" ]
      [ "unicode-script" ]
      [ "unicode-segment" ]
    ];
    dependencies = {
      aho_corasick = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".aho-corasick."1.1.2" { inherit profileName; }).out;
      memchr = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".memchr."2.7.1" { inherit profileName; }).out;
      regex_automata = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".regex-automata."0.4.3" { inherit profileName; }).out;
      regex_syntax = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".regex-syntax."0.8.2" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".regex-automata."0.4.3" = overridableMkRustCrate (profileName: rec {
    name = "regex-automata";
    version = "0.4.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5f804c7828047e88b2d32e2d7fe5a105da8ee3264f01902f796c8e067dc2483f"; };
    features = builtins.concatLists [
      [ "alloc" ]
      [ "dfa-onepass" ]
      [ "hybrid" ]
      [ "meta" ]
      [ "nfa-backtrack" ]
      [ "nfa-pikevm" ]
      [ "nfa-thompson" ]
      [ "perf-inline" ]
      [ "perf-literal" ]
      [ "perf-literal-multisubstring" ]
      [ "perf-literal-substring" ]
      [ "std" ]
      [ "syntax" ]
      [ "unicode" ]
      [ "unicode-age" ]
      [ "unicode-bool" ]
      [ "unicode-case" ]
      [ "unicode-gencat" ]
      [ "unicode-perl" ]
      [ "unicode-script" ]
      [ "unicode-segment" ]
      [ "unicode-word-boundary" ]
    ];
    dependencies = {
      aho_corasick = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".aho-corasick."1.1.2" { inherit profileName; }).out;
      memchr = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".memchr."2.7.1" { inherit profileName; }).out;
      regex_syntax = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".regex-syntax."0.8.2" { inherit profileName; }).out;
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".regex-syntax."0.8.2" = overridableMkRustCrate (profileName: rec {
    name = "regex-syntax";
    version = "0.8.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "c08c74e62047bb2de4ff487b251e4a92e24f48745648451635cec7d591162d9f"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
      [ "unicode" ]
      [ "unicode-age" ]
      [ "unicode-bool" ]
      [ "unicode-case" ]
      [ "unicode-gencat" ]
      [ "unicode-perl" ]
      [ "unicode-script" ]
      [ "unicode-segment" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".tree-sitter."0.20.10" = overridableMkRustCrate (profileName: rec {
    name = "tree-sitter";
    version = "0.20.10";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e747b1f9b7b931ed39a548c1fae149101497de3c1fc8d9e18c62c1a66c683d3d"; };
    dependencies = {
      regex = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".regex."1.10.2" { inherit profileName; }).out;
    };
    buildDependencies = {
      cc = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.83" { profileName = "__noProfile"; }).out;
    };
  });
  
  "unknown".tree-sitter-pie."0.0.1" = overridableMkRustCrate (profileName: rec {
    name = "tree-sitter-pie";
    version = "0.0.1";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      tree_sitter = (rustPackages."registry+https://github.com/rust-lang/crates.io-index".tree-sitter."0.20.10" { inherit profileName; }).out;
    };
    buildDependencies = {
      cc = (buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.83" { profileName = "__noProfile"; }).out;
    };
  });
  
}
