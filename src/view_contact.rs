use std::collections::HashMap;
use std::io::{self, Write};
// 연락처의 전화번호를 보는 함수
pub fn view_contact(contacts: &HashMap<String, String>) {
    print!("Enter name to search: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    
    // 연락처를 검색합니다
    match contacts.get(name.trim()) {
        Some(phone) => println!("Phone: {}", phone),
        None => println!("Contact not found!"),
    }
}