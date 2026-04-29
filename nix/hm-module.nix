{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.sctd;

  # sctd accepts decimal numbers; toString preserves them faithfully.
  args = [
    "--latitude"
    (toString cfg.latitude)
    "--longitude"
    (toString cfg.longitude)
  ]
  ++ cfg.extraArgs;

  exe = lib.getExe cfg.package;
in
{
  options.services.sctd = {
    enable = lib.mkEnableOption "sctd, the set color temperature daemon";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.sctd;
      defaultText = lib.literalExpression "pkgs.sctd";
      description = ''
        The sctd package to use. By default this expects the flake's
        overlay to be applied (so `pkgs.sctd` exists), or you can pass
        the package directly:
          services.sctd.package = inputs.sctd.packages.''${pkgs.system}.default;
      '';
    };

    latitude = lib.mkOption {
      type = lib.types.numbers.between (-90.0) 90.0;
      example = 53.3498;
      description = ''
        Latitude of your location, in decimal degrees.
        Positive = North, negative = South.
      '';
    };

    longitude = lib.mkOption {
      type = lib.types.numbers.between (-180.0) 180.0;
      example = -6.2603;
      description = ''
        Longitude of your location, in decimal degrees.
        Positive = East, negative = West.
      '';
    };

    extraArgs = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = lib.literalExpression ''[ "--verbose" ]'';
      description = "Additional command-line arguments passed to sctd.";
    };

    resetOnStop = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = ''
        When the service stops (e.g. on logout or `systemctl --user stop sctd`),
        run `sctd --reset` to restore the screen's default color temperature.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = pkgs.stdenv.hostPlatform.isLinux;
        message = "services.sctd is only supported on Linux (X11).";
      }
    ];

    home.packages = [ cfg.package ];

    systemd.user.services.sctd = {
      Unit = {
        Description = "sctd — set color temperature daemon";
        Documentation = [ "https://github.com/amir/sctd" ];
        PartOf = [ "graphical-session.target" ];
        After = [ "graphical-session.target" ];
      };

      Service = {
        ExecStart = "${exe} ${lib.concatStringsSep " " (map lib.escapeShellArg args)}";
        ExecStop = lib.mkIf cfg.resetOnStop "${exe} --reset";
        Restart = "on-failure";
        RestartSec = 5;
      };

      Install.WantedBy = [ "graphical-session.target" ];
    };
  };
}
