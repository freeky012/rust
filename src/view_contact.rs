use std::collections::HashMap;
use std::io::{self, Write};

use crate::Contact;

// 연락처의 정보를 보는 함수
pub fn view_contact(contacts: &HashMap<String, Contact>) {
    print!("Enter name to search: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    
    // 연락처를 검색합니다
    match contacts.get(name.trim()) {
        Some(contact) => println!("Phone: {}, Address: {}", contact.phone, contact.address),
        None => println!("Contact not found!"),
    }
}