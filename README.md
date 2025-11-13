# Soroban Price Ticker Contract

This repository contains a simple "Price Ticker" smart contract built with Rust for the Soroban platform on the Stellar network.

## Project Purpose

The primary goal of this project is to demonstrate a fundamental smart contract concept: **authorization and access control**.

## Core Functionality

This contract is designed to store and manage a single price value (e.g., the price of an asset like XLM/USD, represented as a `u64` integer).

It exposes three main functions:

* **`init(admin: Address, start_price: u64)`**: An initialization function that is called once after deployment. It permanently sets the `admin` address for the contract and establishes the initial price.
* **`set_price(new_price: u64)`**: A restricted function that allows the price to be updated. This function can **only** be successfully called by the `admin` address that was set during initialization (it uses `admin.require_auth()`).
* **`get_price()`**: A public, read-only function that allows anyone to query the contract and retrieve the current price.
