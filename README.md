# Crabwatch

<p align="center">
    <img src="https://github.com/user-attachments/assets/294ad962-7ff5-45eb-930e-eab71996558a" width="200" />
</p>

A guide to building a Rust &amp; Raspberry Pi-based open-source monitoring
station.

## What is a monitoring station?

A monitoring station is a physical device that features different displays,
indicators (like LEDs) and displays in real-time the status of a system or a
service. It can be used to monitor the status of a server, a network, a
service, or anything else that can be monitored.

The goal here is not to build particular monitoring station, but to go through
the steps of how I built mine, so you can build your own, or simply learn from
it.

## Scope & Goals

There are multiple goals to this repository:

1. This project allows me to unify all my hobbies (electronics, programming, woodworking) into one realization.
2. It allows me to learn, discover and practice new things like ARM no-std cross-compilation of Rust, low-level hardware protocols (SPI, GPIO, I2C, ...).
3. Getting, in the hand a functional and useful monitoring station that I can hook-up to my job's services monitoring stack.
4. Sharing my discoveries and progress in the hope that it helps or inspires someone else!
5. Having fun!

## Organization

This repository is organized in different sections. Here is an overview of the
layout:

- The [Chapters](./chapters/) folder contains the main content of the project.
  This is likely where you want to start.
- The [Resources](./resources/) folder contains interesting resources that
  helped me while designing and researching everything. Check it out if you
  ever need to dig deeper than what I've done on some aspects!
- The [Sources](./sources/) folder contains the source code of the project. It is divided into two parts:
    - The [Crabwatch](./sources/crabwatch) folder contains the main Rust project, which is the actual final code I use in my real-life project.
    - The [Chapters](./sources/chapters) folder contains the code snippets that are used in the chapters. It is organized in a way that mirrors the chapters, so you can easily find the code that goes with a specific chapter.
