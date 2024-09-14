# Chapter 1 - Getting started!

Our goal is to build a monitoring system based on a Raspberry Pi. We will want
to write, compile and execute some Rust code on it.

This chapter is about setting everything up so that we have a complete
iteration loop at our disposal.

At the end of that chapter, we will have:

- A Raspberry Pi running and connected to our network.
- A way to to cross-compile Rust code locally and deploy it on the Raspberry
  Pi. It won't do anything fancy (yet) but that will get us started.
- The ability to connect to the Raspberry Pi remotely and execute commands on
  it, including our own Rust program.

Let's get into it!

## Glossary

1. [Install a Raspberry Pi](#install-a-raspberry-pi)
2. [Install WSL on Windows](#install-wsl-on-windows)
    1. [Oh-my-zsh](#oh-my-zsh)
3. [IDE](#ide)
4. [General purpose tools](#general-purpose-tools)
5. [Clone this project](#clone-this-project)
6. [Install Rust](#install-rust)
7. [Create a new Rust project](#create-a-new-rust-project)

## Install a Raspberry Pi

The first step is going to be to buy and setup a Raspberry Pi with basic
network connectivity. If budget allows, I recommend getting the best model you
can buy which at the time of writing is the Raspberry Pi 5. Pretty much all we
will do here would work on an older model too, but the more recent models are
faster and have more memory which is definitely more pleasant to work with.

Buy a Raspberry Pi, a power supply, a microSD card and - if you feel like it -
a case. The most recent Raspberry Pi model sometimes come with cooling
radiators which are nice to have.

Installing the Raspberry Pi is pretty straightforward. You can follow the
[official guide](https://www.raspberrypi.com/software/) which is very well
written so there isn't really a need for me to expand on this here.

Just make sure that you:

- Select the Raspberry Pi OS - based on Debian - as the operating system. The
  rest of this guide will assume you are using this OS.
- Connect the Raspberry Pi to your network (either via Ethernet - if you have
  an Ethernet port on your particular model - or WiFi).
- **Enable SSH** so that you can connect to it remotely.

At the end of this step, your Raspberry Pi should be up and running and if you
hooked it up to a monitor, you should be able to see its IP address briefly in
the top right corner notification area. **Write it down: we will need it very
soon**.

## Install WSL on Windows

If you haven't already, you will need to install WSL on your Windows machine.
Follow the [official
guide](https://learn.microsoft.com/en-us/windows/wsl/install) to do so.

At the end of this step, you should have a working WSL2 installation with the
latest Ubuntu LTS distribution.

You can check that everything is working by opening a WSL terminal and running:

```bash
$ lsb_release -a
No LSB modules are available.
Distributor ID: Ubuntu
Description:    Ubuntu 24.04.4 LTS
Release:        24.04
Codename:       noble
```

You may want to use the same version I'm using here to avoid any potential
issue with packages versions.

### Oh-my-zsh

This step is **completely optional** but I highly recommend installing
`oh-my-zsh` to have a more pleasant terminal experience.

Check [the official guide](https://ohmyz.sh/#install) or simply run the
following commands:

```bash
sudo apt update
sudo apt install -y zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
```

You will be prompted to accept setting `zsh` as your default shell. Accept.

Close and reopen your terminal and you should now have a nice prompt!

### Set up SSH

Your Raspberry Pi should be running and connected to your network.

Edit the `~/.ssh/config` file and add the following lines:

```bash
Host pi
    HostName <your-raspberry-ip>
    User pi
```

**Replace `<your-raspberry-ip>` with the IP address you noted down earlier and
adjust the username if you changed it during the Raspberry Pi setup.**

Now we will generate an SSH key and copy its public part onto the Raspberry Pi
to be able to connect to it without typing a password every time:

```bash
ssh-keygen -t rsa -b 4096
ssh-copy-id pi
```

You will be prompted for the password of the `pi` user on the Raspberry Pi.
Once the command is done, you should be able to connect to the Raspberry Pi
without a password:

```bash
ssh pi
hostname # This should print the hostname of your Raspberry Pi.
```

If that works, quit the SSH session by typing `exit` (or pressing `Ctrl+D`).
You should now be back on your local machine, in your WSL terminal.

### IDE

If you intend to use Visual Studio Code, you can install it now. It embeds a
terminal emulator to and can be used to connect to WSL using the [`Remote -
WSL` extension](https://code.visualstudio.com/docs/remote/wsl). Install it.

If you intend to use NeoVim, you can install it now. I won't go into the
details of doing so.

### General purpose tools

You will need to install a bunch of tools and libraries to be able to compile
everything. Run the following commands:

```bash
sudo apt update
sudo apt upgrade
sudo apt install -y build-essential git curl wget
```

### Clone this project

You can now clone this project if you ever need some of its content as a
reference. Run the following commands:

```bash
mkdir -p ~/Projects
cd ~/Projects
git clone https://github.com/ereOn/crabwatch.git
```

### Install Rust

These instructions are a minimal excerpt of the [official Rust installation
guide](https://www.rust-lang.org/tools/install).

Run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then select `1 - Proceed with stable installation` and follow the instructions.

This will download and install the Rust toolchain on your machine, for your
current architecture.

At the end of the installation, you are advised to add the following line to
your shell configuration file:

```bash
echo 'source $HOME/.cargo/env' >> ~/.zshrc
source ~/.zshrc
```

If everything went well, you should now have the `rustc`, `cargo` and `rustup`
commands available in your terminal.

Try running:

```bash
$ rustc --version
rustc 1.80.1 (3f5fd8dd4 2024-08-06)
```

*Note: the specific version number may differ by the time you read this. As long
as your are using a recent version of Rust - at least 1.80.1 - you should be
fine.*

Now is a good time to install some useful Rust-based tools:

```bash
cargo install just # A modern `make` replacement. I use it in this project.
```

We know need to install the cross-compilation toolchain for the Raspberry Pi.

The instructions that follow, I took from this excellent [blog
post](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/).

First install the armv7 Rust toolchain:

```bash
rustup target add armv7-unknown-linux-gnueabihf # For the Raspberry Pi 3 & 4
rustup target add aarch64-unknown-linux-gnu # For the Raspberry Pi 5
```

Then install the cross-compilation toolchain:

```bash
wget -q -O- https://developer.arm.com/-/media/Files/downloads/gnu-a/10.3-2021.07/binrel/gcc-arm-10.3-2021.07-x86_64-arm-none-linux-gnueabihf.tar.xz?rev=302e8e98351048d18b6f5b45d472f406&hash=95ED9EEB24EAEEA5C1B11BBA864519B2 | tar -x -C /opt/
sudo chown -R root: /opt/gcc-arm-10.3-2021.07-x86_64-arm-none-linux-gnueabihf
```

Finally, add the toolchain to your path:

```bash
echo 'export PATH=/opt/gcc-arm-10.3-2021.07-x86_64-arm-none-linux-gnueabihf/bin:$PATH' >> ~/.zshrc
source ~/.zshrc
```

And that's it! That should be it for the prerequisites. We can now move on to
coding some Rust for the Raspberry Pi!

### Create a new Rust project

*Note: the result of this step can be found in the
[`sources/chapters/01-getting_started`](/sources/chapters/01-getting-started)
folder.*

Create a new Rust project:

```bash
cargo new hello-raspberry
cd hello-raspberry
```

Then configure it for cross-compilation:

```bash
mkdir .cargo
cat <<EOF > .cargo/config
[build]
# Set default target for cargo build
target = "armv7-unknown-linux-gnueabihf"
rustflags = ["-C", "linker=arm-none-linux-gnueabihf-gcc"]

[target.armv7-unknown-linux-gnueabihf]
linker = "arm-none-linux-gnueabihf-gcc"
EOF
```

And with this, you should be able to compile your Rust code for the Raspberry
Pi:

```bash
cargo build # Or the shorter `cargo b`
```

This will generate a binary in the `target/armv7-unknown-linux-gnueabihf/debug`
folder which you can copy to your Raspberry Pi (**you will now need its IP
address that you noted down earlier**):

```bash
scp target/armv7-unknown-linux-gnueabihf/debug/hello-raspberry pi:
```

You can now connect to your Raspberry Pi and run the binary:

```bash
$ ssh pi
$ ./hello-raspberry
Hello, world!
```
