use std::vec;

use copper::lsm_tree::{
    self,
    column::Column,
    get_result,
    tree::{self, LsmTree, Value},
};

#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

fn main() {
    print!("{}[2J", 27 as char);
    println!("==== Welcome to Library Manager! ====");
    println!("Please select a shop or create another one:");

    // Ensure the shop folder exists
    let _ = std::fs::create_dir_all("shops");

    // List the shops
    let shops = std::fs::read_dir("shops").unwrap();
    println!("0. Create a new shop");
    for (i, shop) in shops.enumerate() {
        let shop = shop.unwrap();
        println!("{}. {}", i + 1, shop.file_name().into_string().unwrap());
    }

    // Get the user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    print!("{}[2J", 27 as char);
    // Check what user wants
    let mut selected_shop;
    if input == "0" {
        selected_shop = create_shop();
    } else {
        let shops = std::fs::read_dir("shops").unwrap();
        let shop = shops.enumerate().find(|(i, _)| i + 1 == input.parse().unwrap()).unwrap().1;
        let shop = shop.unwrap();
        let shop_name = shop.file_name().into_string().unwrap();
        println!("You selected the shop: {}", shop_name);

        // Shop path
        let shop_path = format!("shops/{}", shop_name);

        selected_shop = tree::LsmTree::load(shop_path).unwrap();
        println!("Shop loaded successfully!");
    }
    println!();
    // Main loop
    loop {
        println!("Please select an option:");
        println!("1. Add a book");
        println!("2. Remove a book");
        println!("3. List all books");
        println!("4. Execute SQL query (advanced users)");
        println!("5. Debug print (advanced users)");
        println!("6. Exit");

        // Get the user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Check what user wants
        print!("{}[2J", 27 as char);
        match input {
            "1" => add_book(&mut selected_shop),
            "2" => remove_book(&mut selected_shop),
            "3" => list_books(&selected_shop),
            "4" => execute_sql_query(&selected_shop),
            "5" => println!("{:#?}", selected_shop),
            "6" => break,
            _ => println!("Invalid option!"),
        }
        println!();
    }

    println!("Goodbye!");
}

fn add_book(shop: &mut LsmTree) {
    // Get book name
    println!("Enter the name of the book:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    // Get author name
    println!("Enter the name of the author:");
    let mut author = String::new();
    std::io::stdin().read_line(&mut author).unwrap();
    let author = author.trim();

    // Get year of publication
    println!("Enter the year of publication:");
    let mut year = String::new();
    std::io::stdin().read_line(&mut year).unwrap();
    let year: i32 = year.trim().parse().unwrap();

    // Add the book
    let key = name.as_bytes();
    let values = vec![name.as_bytes().to_vec(), author.as_bytes().to_vec(), year.to_ne_bytes().to_vec(), b"\x01".to_vec()];
    let _ = shop.insert(key, &values);
    println!("Book added successfully!");
    let mut table = Table::new();
    table.add_row(row!["Name", "Author", "Year", "In Stock"]);
    table.add_row(row![name, author, year, false]);
    table.printstd();
}

fn remove_book(shop: &mut LsmTree) {
    println!("Enter the name of the book to remove:");

    // Get book name
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    // Remove the book
    let key = name.as_bytes();
    let result = shop.delete(key).unwrap();
    if result {
        println!("Book removed successfully!");
    } else {
        println!("Book not found!");
    }
}

fn list_books(shop: &LsmTree) {
    let mut table = Table::new();
    table.add_row(row!["Name", "Author", "Year", "In Stock"]);

    // Get all book with get_range returning always true for the predicate
    let books = shop.get_range(|_| true).unwrap();
    for book in books {
        let decode = shop.decode(&book);
        let name = decode.get("Name").unwrap();
        let author = decode.get("Author").unwrap();
        let year = decode.get("Year").unwrap();
        let in_stock = decode.get("In Stock").unwrap();

        // For each field, match it to a Value type and get the value and add it to the table
        let mut row: Vec<Cell> = vec![];
        match name {
            Value::Text(name) => {
                row.push(Cell::new(name));
            }
            _ => {
                row.push(Cell::new("Unknown"));
            }
        }

        match author {
            Value::Text(author) => {
                row.push(Cell::new(author));
            }
            _ => {
                row.push(Cell::new("Unknown"));
            }
        }

        match year {
            Value::Int(year) => {
                row.push(Cell::new(&year.to_string()));
            }
            _ => {
                row.push(Cell::new("Unknown"));
            }
        }

        match in_stock {
            Value::Bool(in_stock) => {
                row.push(Cell::new(&in_stock.to_string()));
            }
            _ => {
                row.push(Cell::new("Unknown"));
            }
        }

        table.add_row(Row::new(row));
    }

    table.printstd();
}

fn execute_sql_query(shop: &LsmTree) {}

fn create_shop() -> LsmTree {
    println!("Enter the name of the shop:");
    let mut shop_name = String::new();
    std::io::stdin().read_line(&mut shop_name).unwrap();
    let shop_name = shop_name.trim();

    // Create the shop
    let columns = vec![Column::new("Name".to_string(), lsm_tree::column::DataType::Text), Column::new("Author".to_string(), lsm_tree::column::DataType::Text), Column::new("Year".to_string(), lsm_tree::column::DataType::Int), Column::new("In Stock".to_string(), lsm_tree::column::DataType::Bool)];

    // Shop path
    let shop_path = format!("shops/{}", shop_name);

    let shop = tree::LsmTree::new(shop_path, columns);
    println!("Shop created successfully!");

    shop
}

fn test() {
    println!("Testing copper.");

    // Delete the previous LSM tree.
    let _ = std::fs::remove_dir_all("debug_lsm_tree");

    // Create a LSM tree.
    let columns = vec![Column::new("Name".to_string(), lsm_tree::column::DataType::Text), Column::new("Age".to_string(), lsm_tree::column::DataType::Int)];
    let mut lsm_tree = tree::LsmTree::new("debug_lsm_tree".to_string(), columns);

    // Print the LSM tree.
    println!("{:#?}", lsm_tree);

    // Insert a key-value pair.
    let key = "John".as_bytes();
    let values = vec!["John".as_bytes().to_vec(), 42_i32.to_ne_bytes().to_vec()];
    let _ = lsm_tree.insert(key, &values);

    // Print the LSM tree.
    println!("{:#?}", lsm_tree);

    // Get the value of a key.
    let get_result = lsm_tree.get(key);
    let get_result = get_result.unwrap();
    println!("{:?}", get_result);

    // Decode the value.
    let decoded = lsm_tree.decode(&get_result.unwrap());
    println!("{:?}", decoded);

    // Fill the memtable.
    let key = "Jane".as_bytes();
    let values = vec!["Jane".as_bytes().to_vec(), 42_i32.to_ne_bytes().to_vec()];
    let _ = lsm_tree.insert(key, &values);

    let key = "Garry".as_bytes();
    let values = vec!["Garry".as_bytes().to_vec(), 21_i32.to_ne_bytes().to_vec()];
    let _ = lsm_tree.insert(key, &values);

    let key = "Trinity".as_bytes();
    let values = vec!["Trinity".as_bytes().to_vec(), 22_i32.to_ne_bytes().to_vec()];
    let _ = lsm_tree.insert(key, &values);

    /*
       let key = "4".as_bytes();
       let values = vec!["Smogovich".as_bytes().to_vec(), 122_i32.to_ne_bytes().to_vec()];
       let _ = lsm_tree.insert(key, &values);
    */
    // Print the LSM tree.
    println!("{:#?}", lsm_tree);

    // Get a flushed value
    let get_result = lsm_tree.get("Jane".as_bytes());
    let get_result = get_result.unwrap();
    println!("{:?}", get_result);

    // Decode the value.
    let decoded = lsm_tree.decode(&get_result.unwrap());
    println!("{:?}", decoded);

    // Delete John
    let _ = lsm_tree.delete("Jane".as_bytes());
    println!("{:#?}", lsm_tree);

    // Check
    let get_result = lsm_tree.get("Jane".as_bytes());
    let get_result = get_result;
    println!("{:?}", get_result);
}
