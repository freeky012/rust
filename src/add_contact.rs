use std::collections::HashMap;
use std::io::{self, Write};
// 새 연락처를 추가하는 함수
#[allow(dead_code)]
pub fn add_contact(contacts: &mut HashMap<String, String>) {
    print!("Enter name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    
    print!("Enter phone: ");
    io::stdout().flush().unwrap();
    let mut phone = String::new();
    io::stdin().read_line(&mut phone).unwrap();
    
    // HashMap에 연락처를 삽입합니다
    contacts.insert(name.trim().to_string(), phone.trim().to_string());
    println!("Contact added!");
}