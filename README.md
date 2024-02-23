# OrganiksICP 🦀📄

## Overview 🌐

This Rust smart contract is developed for managing records related to poultry, eggs, egg orders, and egg prices on the Internet Computer platform. It provides a set of functionalities for adding, updating, retrieving, and deleting records associated with poultry farming..

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with organiks, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd organiks/
dfx help
dfx canister --help
```


## Dependencies 📦

- **serde**: A Rust library for serialization and deserialization.
- **candid**: A library for Candid, a serialization and Interface Description Language (IDL) used on the Internet Computer.
- **ic_cdk**: The Internet Computer Common Development Kit, facilitating interaction with the Internet Computer platform.
- **ic_stable_structures**: A library providing stable storage structures for persistent data storage.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

## Data Structures 🛠️

### Enum

#### `EggType` 🥚

- Custom enum representing the type of egg.
- Variants: `Kienyeji`, `Grade`.

### Structs

#### `PoultryRecord` 🐔

- Represents a poultry record.
- Fields:
  - `id`: Unique identifier for the record.
  - `breed`: String representing the breed of the poultry.
  - `age`: Age of the poultry in months.
  - `egg_production`: Boolean indicating whether the poultry produces eggs.
  - `created_at`: Timestamp indicating when the record was created.
  - `updated_at`: Optional timestamp indicating when the record was last updated.

#### `EggRecord` 🥚📜

- Represents an egg record.
- Fields:
  - `id`: Unique identifier for the record.
  - `egg_type`: Type of the egg (`Kienyeji` or `Grade`).
  - `total_egg_count`: Total count of eggs.
  - `cracked_egg_count`: Count of cracked eggs.
  - `created_at`: Timestamp indicating when the record was created.
  - `updated_at`: Optional timestamp indicating when the record was last updated.

#### `EggOrder` 🛒🥚

- Represents an egg order.
- Fields:
  - `id`: Unique identifier for the order.
  - `customer_name`: String representing the name of the customer.
  - `egg_type`: Type of the egg in the order.
  - `quantity`: Quantity of eggs in the order.
  - `total_price`: Total price of the order.
  - `created_at`: Timestamp indicating when the order was placed.

#### `EggPrice` 💰🥚

- Represents an egg price.
- Fields:
  - `id`: Unique identifier for the price entry.
  - `egg_type`: Type of the egg for which the price is set.
  - `price`: Price of the egg.

### Traits 🧬

#### `Storable` 🗄️

- Trait defining methods for converting a struct to bytes and vice versa.
- Implemented for `PoultryRecord`, `EggRecord`, `EggOrder`, and `EggPrice`.

#### `BoundedStorable` 📏🗄️

- Additional trait for structs that are stored in a stable struct, providing information about maximum size and whether the size is fixed.
- Implemented for `PoultryRecord`, `EggRecord`, `EggOrder`, and `EggPrice`.

## Memory Management 🧠

### Memory

- Type alias for `VirtualMemory` using `DefaultMemoryImpl`.

### IdCell

- Type alias for `Cell` holding a u64 within the specified `Memory`.

### MemoryManager

- Manages memory using `DefaultMemoryImpl`.

### Thread-Local Memory 🧵

- `MEMORY_MANAGER`: Thread-local memory manager holding `MemoryManager<DefaultMemoryImpl>`.
- `ID_COUNTER`: Thread-local counter for generating unique IDs.
- `STORAGE`: Thread-local stable BTree map for poultry records.
- `EGG_STORAGE`: Thread-local stable BTree map for egg records.
- `ORDER_STORAGE`: Thread-local stable BTree map for egg orders.
- `EGG_PRICE_STORAGE`: Thread-local stable BTree map for egg prices.

## Payload Structs 📦

### Poultry Record Payload 🐔📦

- Struct used as a payload for adding poultry records.
- Fields:
  - `breed`: String representing the breed of the poultry.
  - `age`: Age of the poultry in months.
  - `egg_production`: Boolean indicating whether the poultry produces eggs.

### Egg Record Payload 🥚📜📦

- Struct used as a payload for adding egg records.
- Fields:
  - `egg_type`: Type of the egg (`Kienyeji` or `Grade`).
  - `total_egg_count`: Total count of eggs.
  - `cracked_egg_count`: Count of cracked eggs.

### Egg Order Payload 🛒🥚📦

- Struct used as a payload for placing egg orders.
- Fields:
  - `customer_name`: String representing the name of the customer.
  - `egg_type`: Type of the egg in the order.
  - `quantity`: Quantity of eggs in the order.

### Egg Price Payload 💰🥚📦

- Struct used as a payload for setting egg prices.
- Fields:
  - `egg_type`: Type of the egg for which the price is set.
  - `price`: Price of the egg.

## Update Methods 🔄

### Poultry Record

#### `add_poultry_record(payload: PoultryRecordPayload) -> Option<PoultryRecord>` 🐔📝

- Adds a new poultry record.

#### `update_poultry_record(id: u64, payload: PoultryRecordPayload) -> Result<PoultryRecord, Error>` 🐔🔄

- Updates an existing poultry record.

#### `delete_poultry_record(id: u64) -> Result<PoultryRecord, Error>` 🐔🗑️

- Deletes a poultry record.

### Egg Record

#### `add_egg_record(payload: EggRecordPayload) -> Option<EggRecord>` 🥚📝

- Adds a new egg record.

#### `update_egg_record(id: u64, payload: EggRecordPayload) -> Result<EggRecord, Error>` 🥚🔄

- Updates an existing egg record.

#### `delete_egg_record(id: u64) -> Result<EggRecord, Error>` 🥚🗑️

- Deletes an egg record.

### Egg Price

#### `set_egg_price(payload: EggPricePayload) -> Option<EggPrice>` 💰🥚📝

- Sets the price for a specific type of egg.

#### `update_egg_price(id: u64, payload: EggPricePayload) -> Result<EggPrice, Error>` 💰🥚🔄

- Updates the price for a specific type of egg.

#### `delete_egg_price(id: u64) -> Result<EggPrice, Error>` 💰🥚🗑️

- Deletes the price entry for a specific type of egg.

### Egg Order

#### `place_egg_order(payload: EggOrderPayload) -> Result<EggOrder, Error>` 🛒🥚📝

- Places a new egg order.

## Query Methods 🔍

### Poultry Record

#### `get_poultry_record(id: u64) -> Result<PoultryRecord, Error>` 🐔🔍

- Retrieves a poultry record by ID.

#### `get_all_poultry_records() -> Result<Vec<PoultryRecord>, Error>` 🐔🔍

- Retrieves all poultry records.

### Egg Record

#### `get_egg_record(id: u64) -> Result<EggRecord, Error>` 🥚📜🔍

- Retrieves an egg record by ID.

#### `search_egg_record_by_egg_type(egg_type: EggType) -> Result<Vec<EggRecord>, Error>` 🥚📜🔍

- Retrieves egg records by egg type.

#### `get_all_egg_records() -> Result<Vec<EggRecord>, Error>` 🥚📜🔍

- Retrieves all egg records.

### Egg Price

#### `get_egg_price(id: u64) -> Result<EggPrice, Error>` 💰🥚🔍

- Retrieves an egg price by ID.

#### `get_egg_price_by_egg_type(egg_type: EggType) -> Result<Vec<EggPrice>, Error>` 💰🥚🔍

- Retrieves egg prices by egg type.

#### `get_all_egg_prices() -> Result<Vec<EggPrice>, Error>` 💰🥚🔍

- Retrieves all egg prices.

### Egg Order

#### `get_egg_order(id: u64) -> Result<EggOrder, Error>` 🛒🥚🔍

- Retrieves an egg order by ID.

#### `get_all_orders() -> Result<Vec<EggOrder>, Error>` 🛒🥚🔍

- Retrieves all egg orders.

## Error Handling 🚨

### `Error` Enum

- Enum representing possible error scenarios.
- Variants:
  - `NotFound`: Indicates that the requested resource was not found.
  - `msg`: String providing additional information about the error.

## Exported Candid Methods 🚀

- Methods are annotated with `ic_cdk::update` or `ic_cdk::query` for proper exposure to the Internet Computer platform.

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
