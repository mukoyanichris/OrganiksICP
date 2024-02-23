#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// EggType is a custom enum type that is used to represent the type of egg.
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum EggType {
    #[default]
    Kienyeji,
    Grade,
}

// PoultryRecord is a struct that represents a poultry record.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct PoultryRecord {
    id: u64,
    breed: String,
    age: u32,
    egg_production: bool,
    created_at: u64,
    updated_at: Option<u64>,
}

// EggRecord is a struct that represents an egg record.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EggRecord {
    id: u64,
    egg_type: EggType,
    total_egg_count: u32,
    cracked_egg_count: u32,
    created_at: u64,
    updated_at: Option<u64>,
}

// EggOrder is a struct that represents an egg order.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EggOrder {
    id: u64,
    customer_name: String,
    egg_type: EggType,
    quantity: u32,
    total_price: f64,
    created_at: u64,
}

// EggPrice is a struct that represents an egg price.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EggPrice {
    id: u64,
    egg_type: EggType,
    price: f64,
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for PoultryRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for PoultryRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for EggRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for EggRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for EggOrder {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EggOrder {
    const MAX_SIZE: u32 = 1024; // Adjust the size based on your requirements
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for EggPrice {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EggPrice {
    const MAX_SIZE: u32 = 1024; // Adjust the size based on your requirements
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    // Poultry Storage
    static STORAGE: RefCell<StableBTreeMap<u64, PoultryRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    
    // Egg Storage
    static EGG_STORAGE: RefCell<StableBTreeMap<u64, EggRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
    
    // Egg Order
    static ORDER_STORAGE: RefCell<StableBTreeMap<u64, EggOrder, Memory>> =
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
    
    // Egg Price
    static EGG_PRICE_STORAGE: RefCell<StableBTreeMap<u64, EggPrice, Memory>> =
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

// Poultry Record Payload
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct PoultryRecordPayload {
    breed: String,
    age: u32,
    egg_production: bool,
}

// Egg Record Payload
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct EggRecordPayload {
    egg_type: EggType,
    total_egg_count: u32,
    cracked_egg_count: u32,
}

// Egg Order Payload
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct EggOrderPayload {
    customer_name: String,
    egg_type: EggType,
    quantity: u32,
}

// Egg Price Payload
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct EggPricePayload {
    egg_type: EggType,
    price: f64,
}

// Poultry Record

// Add a new poultry record
#[ic_cdk::update]
fn add_poultry_record(payload: PoultryRecordPayload) -> Option<PoultryRecord> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let record = PoultryRecord {
        id,
        breed: payload.breed,
        age: payload.age,
        egg_production: payload.egg_production,
        created_at: time(),
        updated_at: None,
    };
    do_insert(&record);
    Some(record)
}

// helper method to perform insert.
fn do_insert(record: &PoultryRecord) {
    STORAGE.with(|service| service.borrow_mut().insert(record.id, record.clone()));
}

// Get a poultry record by id
#[ic_cdk::query]
fn get_poultry_record(id: u64) -> Result<PoultryRecord, Error> {
    match _get_poultry_record(&id) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!("a poultry record with id={} not found", id),
        }),
    }
}

// a helper method to get a poultry record by id. used in get_poultry_record/update_poultry_record
fn _get_poultry_record(id: &u64) -> Option<PoultryRecord> {
    STORAGE.with(|service| service.borrow().get(id))
}

// Search all poultry records
#[ic_cdk::query]
fn get_all_poultry_records() -> Result<Vec<PoultryRecord>, Error> {
    STORAGE.with(|service| {
        let stable_btree_map = &*service.borrow();

        let records: Vec<PoultryRecord> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();

        if records.is_empty() {
            Err(Error::NotFound {
                msg: "No poultry records found.".to_string(),
            })
        } else {
            Ok(records)
        }
    })
}

// Update a poultry record
#[ic_cdk::update]
fn update_poultry_record(id: u64, payload: PoultryRecordPayload) -> Result<PoultryRecord, Error> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut record) => {
            record.breed = payload.breed;
            record.age = payload.age;
            record.egg_production = payload.egg_production;
            record.updated_at = Some(time());
            do_insert(&record);
            Ok(record)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a poultry record with id={}. record not found",
                id
            ),
        }),
    }
}


// Delete a poultry record
#[ic_cdk::update]
fn delete_poultry_record(id: u64) -> Result<PoultryRecord, Error> {
    match STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a poultry record with id={}. record not found.",
                id
            ),
        }),
    }
}

// Egg Record

// Add a new egg record
#[ic_cdk::update]
fn add_egg_record(payload: EggRecordPayload) -> Option<EggRecord> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let record = EggRecord {
        id,
        egg_type: payload.egg_type,
        total_egg_count: payload.total_egg_count,
        cracked_egg_count: payload.cracked_egg_count,
        created_at: time(),
        updated_at: None,
    };
    do_insert_egg(&record);
    Some(record)
}

// helper method to perform insert for egg records.
fn do_insert_egg(record: &EggRecord) {
    EGG_STORAGE.with(|service| service.borrow_mut().insert(record.id, record.clone()));
}

// Get an egg record by id
#[ic_cdk::query]
fn get_egg_record(id: u64) -> Result<EggRecord, Error> {
    match _get_egg_record(&id) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!("an egg record with id={} not found", id),
        }),
    }
}

// a helper method to get an egg record by id. used in get_egg_record/update_egg_record
fn _get_egg_record(id: &u64) -> Option<EggRecord> {
    EGG_STORAGE.with(|service| service.borrow().get(id))
}

// Search egg records by egg type
#[ic_cdk::query]
fn search_egg_record_by_egg_type(egg_type: EggType) -> Result<Vec<EggRecord>, Error> {
    EGG_STORAGE.with(|service| {
        let stable_btree_map = &*service.borrow();

        // Use iter to obtain an iterator over (key, value) pairs
        let records: Vec<EggRecord> = stable_btree_map
            .iter()
            .filter(|(_, record)| record.egg_type == egg_type)
            .map(|(_, record)| record.clone())
            .collect();

        if records.is_empty() {
            Err(Error::NotFound {
                msg: format!("no egg records found for egg type: {:?}", egg_type),
            })
        } else {
            Ok(records)
        }
    })
}


// Search all egg records
#[ic_cdk::query]
fn get_all_egg_records() -> Result<Vec<EggRecord>, Error> {
    EGG_STORAGE.with(|service| {
        let stable_btree_map = &*service.borrow();

        let records: Vec<EggRecord> = stable_btree_map
            .iter()
            .map(|(_, record)| record.clone())
            .collect();

        if records.is_empty() {
            Err(Error::NotFound {
                msg: "No egg records found.".to_string(),
            })
        } else {
            Ok(records)
        }
    })
}

// Update an egg record
#[ic_cdk::update]
fn update_egg_record(id: u64, payload: EggRecordPayload) -> Result<EggRecord, Error> {
    match EGG_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut record) => {
            record.egg_type = payload.egg_type;
            record.total_egg_count = payload.total_egg_count;
            record.cracked_egg_count = payload.cracked_egg_count;
            record.updated_at = Some(time());
            do_insert_egg(&record);
            Ok(record)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update an egg record with id={}. record not found",
                id
            ),
        }),
    }
}

// Delete an egg record
#[ic_cdk::update]
fn delete_egg_record(id: u64) -> Result<EggRecord, Error> {
    match EGG_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete an egg record with id={}. record not found.",
                id
            ),
        }),
    }
}

// Egg Price

// Add a new egg price
#[ic_cdk::update]
fn set_egg_price(payload: EggPricePayload) -> Option<EggPrice> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let egg_price = EggPrice {
        id,
        egg_type: payload.egg_type,
        price: payload.price,
    };
    do_insert_egg_price(&egg_price);
    Some(egg_price)
}

// helper method to perform insert.
fn do_insert_egg_price(egg_price: &EggPrice) {
    EGG_PRICE_STORAGE.with(|service| service.borrow_mut().insert(egg_price.id, egg_price.clone()));
}

// Get an egg price by id
#[ic_cdk::query]
fn get_egg_price(id: u64) -> Result<EggPrice, Error> {
    match _get_egg_price(&id) {
        Some(price) => Ok(price),
        None => Err(Error::NotFound {
            msg: format!("egg price with id={} not found", id),
        }),
    }
}

// a helper method to get an egg price by id. used in get_egg_price/update_egg_price
fn _get_egg_price(id: &u64) -> Option<EggPrice> {
    EGG_PRICE_STORAGE.with(|service| service.borrow().get(id))
}

// Search egg prices by egg type
#[ic_cdk::query]
fn get_egg_price_by_egg_type(egg_type: EggType) -> Result<Vec<EggPrice>, Error> {
    let result: Result<Vec<EggPrice>, Error> = EGG_PRICE_STORAGE.with(|service| {
        let prices: Vec<EggPrice> = service
            .borrow()
            .iter()
            .filter(|(_, v)| v.egg_type == egg_type)
            .map(|(_, v)| v.clone())
            .collect();

        if prices.is_empty() {
            Err(Error::NotFound {
                msg: format!("No egg prices found for egg type: {:?}", egg_type),
            })
        } else {
            Ok(prices)
        }
    });

    result
}

// Search all egg prices
#[ic_cdk::query]
fn get_all_egg_prices() -> Result<Vec<EggPrice>, Error> {
    EGG_PRICE_STORAGE.with(|service| {
        let stable_btree_map = &*service.borrow();

        let prices: Vec<EggPrice> = stable_btree_map
            .iter()
            .map(|(_, price)| price.clone())
            .collect();

        if prices.is_empty() {
            Err(Error::NotFound {
                msg: "No egg prices found.".to_string(),
            })
        } else {
            Ok(prices)
        }
    })
}


// Update an egg price
#[ic_cdk::update]
fn update_egg_price(id: u64, payload: EggPricePayload) -> Result<EggPrice, Error> {
    match EGG_PRICE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut egg_price) => {
            egg_price.egg_type = payload.egg_type;
            egg_price.price = payload.price;
            do_insert_egg_price(&egg_price);
            Ok(egg_price)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update egg price with id={}. price not found", id),
        }),
    }
}

// Delete an egg price
#[ic_cdk::update]
fn delete_egg_price(id: u64) -> Result<EggPrice, Error> {
    match EGG_PRICE_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(egg_price) => Ok(egg_price),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete egg price with id={}. price not found.", id),
        }),
    }
}

// Egg Order

// Place an egg order
#[ic_cdk::update]
fn place_egg_order(payload: EggOrderPayload) -> Result<EggOrder, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    // Fetch the egg price from EGG_PRICE_STORAGE based on the provided egg_type
    let egg_price = EGG_PRICE_STORAGE
        .with(|service| {
            let borrow = service.borrow();
            borrow
                .iter()
                .find(|(_, price)| price.egg_type == payload.egg_type)
                .map(|(_, price)| price.price)
        })
        .ok_or_else(|| Error::NotFound {
            msg: format!("Egg price not found for egg type {:?}", payload.egg_type),
        })?;

    let total_price = egg_price * payload.quantity as f64;

    let order = EggOrder {
        id,
        customer_name: payload.customer_name,
        egg_type: payload.egg_type,
        quantity: payload.quantity,
        total_price,
        created_at: time(),
    };

    ORDER_STORAGE.with(|service| service.borrow_mut().insert(order.id, order.clone()));
    Ok(order)
}

// Get an egg order by id
#[ic_cdk::query]
fn get_egg_order(id: u64) -> Result<EggOrder, Error> {
    ORDER_STORAGE.with(|service| {
        let stable_btree_map = &*service.borrow();

        match stable_btree_map.get(&id) {
            Some(order) => Ok(order.clone()),
            None => Err(Error::NotFound {
                msg: format!("Egg order with id={} not found", id),
            }),
        }
    })
}

// Search all egg orders
#[ic_cdk::query]
fn get_all_orders() -> Result<Vec<EggOrder>, Error> {
    ORDER_STORAGE.with(|service| {
        let stable_btree_map = &*service.borrow();

        // Use iter to obtain an iterator over (key, value) pairs
        let orders: Vec<EggOrder> = stable_btree_map
            .iter()
            .map(|(_, order)| order.clone())
            .collect();

        if orders.is_empty() {
            Err(Error::NotFound {
                msg: "No egg orders found.".to_string(),
            })
        } else {
            Ok(orders)
        }
    })
}

// Error type
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}


ic_cdk::export_candid!();
