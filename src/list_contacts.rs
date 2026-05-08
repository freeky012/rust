use std::collections::HashMap;

use crate::Contact;

// 모든 연락처를 나열하는 함수
pub fn list_contacts(contacts: &HashMap<String, Contact>) {
    if contacts.is_empty() {
        println!("No contacts!");
    } else {
        for (name, contact) in contacts {
            println!("{}: {} - {}", name, contact.phone, contact.address);
        }
    }
}