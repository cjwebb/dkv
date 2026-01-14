# Distributed Key-Value Store (DKV)

## Overview

A thread-per-core rust KV store, built to explore high-performance distributed systems in Rust.

## Design v0.1

- Setup a client and server. Hash keys to each server, and core. Change this later to use better partitioning.
- Implement get/put.
- Keep everything in memory. Data persistence is out of scope for now.
- Focus on server crate.
- Implement a gossip protocol for membership and failure detection.

## TODO

- Need to alter client to write when doing GET, and waiting for a response when doing SET.
- Make workers use BlockStore.
- Implement a protcol (like Redis Protocol?)
- Start caring about performance! This code isn't fast at all right now.

## Longer Term

- Jepsen / Antithesis testing.
