use std::collections::HashMap;
use std::fs;   

use crate::Contact;

const CONTACTS_FILE: &str = "contacts.txt";

// 연락처를 파일에 저장하는 함수
pub fn save_contacts(contacts: &HashMap<String, Contact>) {
    let mut content = String::new();
    // 각 연락처를 "이름:전화번호:주소\n" 형식으로 포맷합니다
    for (name, contact) in contacts {
        content.push_str(&format!("{}:{}:{}\n", name, contact.phone, contact.address));
    }
    // 파일에 씁니다
    let _ = fs::write(CONTACTS_FILE, content);
}
 
