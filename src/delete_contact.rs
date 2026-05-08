use std::collections::HashMap;
use std::io::{self, Write};

use crate::Contact;

// 연락처를 삭제하는 함수
pub fn delete_contact(contacts: &mut HashMap<String, Contact>) {
    print!("Enter name to delete: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    
    // 연락처가 존재하면 제거합니다
    if contacts.remove(name.trim()).is_some() {
        println!("Contact deleted!");
    } else {
        println!("Contact not found!");
    }
}