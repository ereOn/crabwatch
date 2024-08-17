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

You can check that everything is working by opening a terminal and running:

```bash
$ lsb_release -a
No LSB modules are available.
Distributor ID: Ubuntu
Description:    Ubuntu 22.04.4 LTS
Release:        22.04
Codename:       jammy
```
