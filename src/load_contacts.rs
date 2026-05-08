use std::collections::HashMap;
use std::fs;

use crate::Contact;

const CONTACTS_FILE: &str = "contacts.txt";

// 파일에서 연락처를 로드하는 함수
pub fn load_contacts() -> HashMap<String, Contact> {
    let mut contacts = HashMap::new();
    
    // 파일을 읽고 각 줄을 파싱합니다
    if let Ok(content) = fs::read_to_string(CONTACTS_FILE) {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let phone = parts[1].to_string();
                let address = parts[2..].join(":");
                contacts.insert(name, Contact { phone, address });
            }
        }
    }
    
    contacts
}