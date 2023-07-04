+++
title = "Demystifying NixOS: The Basic Flake (1/4)"
date = 2023-06-25
description = """
NixOS can often feel overwhelming, especially for newcomers to the operating system.

In this opinionated series, my aim is to provide a structured path that takes you from a simple NixOS configuration to a more complex one, while explaining the underlying concepts along the way.

In part 1 we look at a basic starting point.
"""

[taxonomies]
tags = ["demystifying-nixos", "nixos"]
+++

NixOS can often feel overwhelming, especially for newcomers to the operating system.

One of the main challenges is finding the right starting point. The available examples on the internet tend to fall into two categories: they either provide overly simplistic configurations that don't cover real-world scenarios, or they present highly complex setups without offering a clear path for beginners to follow.

I believe this is because crafting a NixOS configuration is an iterative process that requires exploration and experimentation. As users gain more experience and knowledge of NixOS and the Nix expression language, they can gradually build more intricate and sophisticated configurations.

In this opinionated series, my aim is to provide a structured path that takes you from a simple NixOS configuration to a more complex one, while explaining the underlying concepts along the way. By the end of this series, you will have a comprehensive understanding of how to create a NixOS configuration that:

- Utilizes flakes.
- Supports multiple hosts.
- Enables the installation of custom packages.
- Can be extended through modules.
- Organizes configuration options into profiles.
- Is fully understood by you, the reader.

## A starting point

Let's begin with a simple NixOS configuration that will serve as our starting point throughout this series. Take a look at the code snippet below:

```nix
# file: flake.nix

{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
  };

  outputs = inputs: {
    nixosConfigurations = {
      myhost = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [ ];
      };
    };
  };
}
```

This `flake.nix` file represents the foundation of our NixOS configuration. It might be simple, but let's take a moment to understand what's happening here.

Within the `inputs` attribute, we specify the source of our Nix packages by providing the URL for the desired version of the Nixpkgs repository. In this example, we're using the `nixos-23.05` branch. You can include additional inputs as needed, but for now, we'll keep it simple.

The `outputs` attribute is a function that takes the `inputs` as an argument and returns a set of well known attributes. In this case, we're only interested in the `nixosConfigurations` output attribute, which is a set of available configurations for different hosts. We define a single host called `myhost`. Please note that the key name might be different from the actual hostname of the machine.

To construct the `myhost` configuration, we utilize the `nixpkgs.lib.nixosSystem` function provided by the Nixpkgs library. This function takes in various parameters, such as the target system architecture (`system`) and a list of modules (`modules`) that define specific configuration settings. In our initial configuration, the modules list is empty, resulting in a default NixOS configuration.

## Modules

To customize our NixOS system we'll create our own modules. Let's take a closer look at their structure:

```nix
{
  imports = [
    # paths to other modules
  ];

  options = {
    # option declarations
  };

  config = {
    # option definitions
  };
}
```

A module consists of three components. First, we have the `imports` attribute, which allows us to include other modules, enabling us to compose our configuration in a modular manner.

Next, we define the `options` attribute, where we declare new configuration options provided by the module. We will delve deeper into this topic in later sections of this series.

Finally, we have the `config` attribute, where we define the actual configuration options to customize our system. This is where we'll spend most of our time.

In many cases, we may only need the option definitions, resulting in a more concise configuration. In these instances, we can omit the `options` attribute and directly define the options at the top level of the module, like so:

```nix
{
  imports = [
    # paths to other modules
  ];

  # option definitions
}
```

Now, let's see how we can apply modules to customize our system. Take a look at the updated `flake.nix` file:

```nix
# file: flake.nix

{
  # ...

  outputs = inputs: {
    nixosConfigurations = {
      myhost = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [{
          users.users.demo.isNormalUser = true;
          services.getty.autologinUser = "demo";
          system.stateVersion = "23.05";
        }];
      };
    };
  };
}
```

We've added a single module to the `modules` list. This module defines basic configuration options, such as creating a user named `demo` and enabling automatic login for that user. 

Additionally, we have specified the desired NixOS state version as `23.05` to suppress the warning message we would otherwise receive.

## Testing it out

Now that we have our basic NixOS configuration in place, it's time to test it out and see it in action. To do this, we can run the following commands:

```bash
rm *.qcow2; nixos-rebuild build-vm --flake .#myhost && result/bin/run-*-vm
```

This command will build a virtual machine image for our `myhost` configuration and run it using QEMU/KVM. If everything goes well, you should be logged in as the `demo` user.

## Splitting the configuration

To enhance organization and modularity, it is often useful to split the configuration into separate files. Let's explore how we can achieve this by moving our configuration to a separate file.

In the `flake.nix` file, we can modify the `modules` list to include a path pointing to a file containing a module definition. Let's create a new file called `configuration.nix` and move our configuration there:

```nix
# file: flake.nix

{
  # ...

  outputs = inputs: {
    nixosConfigurations = {
      myhost = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [ ./configuration.nix ];
      };
    };
  };
}

# file: configuration.nix

{
  users.users.demo.isNormalUser = true;
  services.getty.autologinUser = "demo";
  system.stateVersion = "23.05";
}
```

The split configuration file should produce the same result as before, and we can verify this by running the same command as before. Both evaluations will lead to the same configuration, so Nix should not rebuild anything.

Now, let's consider an updated version of the configuration.nix file that includes an additional package installation:

```nix
# file: configuration.nix

{ pkgs, ... }: {
  users.users.demo.isNormalUser = true;
  services.getty.autologinUser = "demo";
  system.stateVersion = "23.05";

  environment.systemPackages = [
    pkgs.neofetch
  ];
}
```

In this updated version, we've turned our configuration into a function that takes an argument `pkgs`. This enables us to access the package set and install additional packages. In this case, we have added `neofetch` to the `environment.systemPackages` list as an example package.

You might be recognizing this `configuration.nix`; it is indeed the same file as the one created by `nixos-generate-config`! This means that we can move that file here and completely migrate to a flake-based configuration.

## Extra: Home Manager

Many NixOS users also utilize Home Manager to manage their user configurations. While I have some mixed feelings about it, Home Manager serves as an excellent demonstration of how to incorporate additional inputs into our flake.

```nix
# file: flake.nix

{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";

    home-manager = {
      url = "github:nix-community/home-manager/release-23.05";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: {
    nixosConfigurations = {
      myhost = inputs.nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ./configuration.nix
          inputs.home-manager.nixosModules.home-manager
        ];
      };
    };
  };
}
```

To incorporate Home Manager into our configuration, we have added a new input pointing to the branch that matches our Nixpkgs version. Additionally, we set it to use the same Nixpkgs input as our NixOS configuration.

We include the provided Home Manager module by appending `inputs.home-manager.nixosModules.home-manager` to the `modules` list. With this addition, our configuration is now ready to utilize Home Manager.

```nix
# file: configuration.nix

{ pkgs, ... }: {
  users.users.demo.isNormalUser = true;
  services.getty.autologinUser = "demo";
  system.stateVersion = "23.05";

  environment.systemPackages = with pkgs; [
    neofetch
  ];

  home-manager.useGlobalPkgs = true;
  home-manager.useUserPackages = true;
  home-manager.users.demo = {
    home.stateVersion = "23.05";
    programs.newsboat = {
      enable = true;
      urls = [{ url = "https://lhf.pt/atom.xml"; }];
    };
  };
}
```
Within the configuration file, we have enabled Home Manager and added a simple configuration for the demo user. This configuration enables the newsboat program and includes the feed from this blog.

## Next in the series: Multi-Host Configurations

In the next installment of this series, we will explore how to expand our flake configuration to include multiple hosts. We will delve into the power of the Nix expression language and learn how to template our config to generate host configurations dynamically.

Read the next post here: (coming soon)

Read other posts in this series: [Demystifying NixOS](/tags/demystifying-nixos/)
