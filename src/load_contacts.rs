use std::collections::HashMap;
use std::fs;

const CONTACTS_FILE: &str = "contacts.txt";

// 파일에서 연락처를 로드하는 함수
pub fn load_contacts() -> HashMap<String, String> {
    let mut contacts = HashMap::new();
    
    // 파일을 읽고 각 줄을 파싱합니다
    if let Ok(content) = fs::read_to_string(CONTACTS_FILE) {
        for line in content.lines() {
            if let Some(pos) = line.find(':') {
                let name = line[..pos].to_string();
                let phone = line[pos + 1..].trim().to_string();
                contacts.insert(name, phone);
            }
        }
    }
    
    contacts
}