use std::collections::HashMap;

// 모든 연락처를 나열하는 함수
pub fn list_contacts(contacts: &HashMap<String, String>) {
    if contacts.is_empty() {
        println!("No contacts!");
    } else {
        for (name, phone) in contacts {
            println!("{}: {}", name, phone);
        }
    }
}