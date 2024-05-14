use rocksdb::{DB, Options};

static mut DB_INSTANCE: Option<DB> = None;

/*pub fn setup_db(path:&str) -> bool {
    println!("Set up database");
    unsafe { DB_INSTANCE = Some(DB::open_default(path).unwrap()) };
    true
}*/

pub fn open_db(path:&str) -> bool {
    println!("Open database");
    let mut opts = Options::default();
    opts.create_if_missing(true);
    unsafe { DB_INSTANCE = Some(DB::open(&opts, path).unwrap()) };
    true
}

pub fn close_db() -> bool {
    println!("Close database");
    unsafe { drop( DB_INSTANCE.take() )};
    true
}

pub fn destroy_db(path:&str) -> bool {
    println!("Destroy database");
    //let _ = DB::destroy(&Options::default(), path);
    match DB::destroy(&Options::default(), path) {
        Ok(_) => return true,
        Err(_e) => return false,
    }
}

pub fn write_db(key:&str, value:&str) -> bool {
    println!("Write key {}, value {} to database", key, value);
    unsafe { DB_INSTANCE.as_ref().unwrap().put(key, value).unwrap() };
    true
}

pub fn read_db(key:&str) -> (bool, String) {
    println!("Retrieve value for key {} from database", key);
    match unsafe { DB_INSTANCE.as_ref().unwrap().get(key) } {
        Ok(Some(value)) => return (true, String::from_utf8(value).unwrap()),
        Ok(None) => return (false, "key not found".into()),
        Err(e) => return (false, e.into()),
    };
}

pub fn delete_db(key:&str) -> bool {
    println!("Delete key {} from database", key);
    unsafe { DB_INSTANCE.as_ref().unwrap().delete(key).unwrap() };
    true
}