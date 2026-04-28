use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;

const CONTACTS_FILE: &str = "contacts.txt";

fn main() {
    // 파일에서 기존 연락처를 로드합니다
    let mut contacts: HashMap<String, String> = load_contacts();
    
    // 주소록 메뉴를 위한 메인 루프
    loop {
        println!("\n--- Address Book ---");
        println!("1. Add contact");
        println!("2. View contact");
        println!("3. List all contacts");
        println!("4. Delete contact");
        println!("5. Save and exit");
        print!("Choose option: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // 사용자 입력에 따라 작업을 수행합니다
        match input.trim() {
            "1" => add_contact(&mut contacts),
            "2" => view_contact(&contacts),
            "3" => list_contacts(&contacts),
            "4" => delete_contact(&mut contacts),
            "5" => {
                save_contacts(&contacts);
                break;
            },
            _ => println!("Invalid option!"),
        }
    }
}

// 새 연락처를 추가하는 함수
fn add_contact(contacts: &mut HashMap<String, String>) {
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

// 연락처의 전화번호를 보는 함수
fn view_contact(contacts: &HashMap<String, String>) {
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

// 모든 연락처를 나열하는 함수
fn list_contacts(contacts: &HashMap<String, String>) {
    if contacts.is_empty() {
        println!("No contacts!");
    } else {
        for (name, phone) in contacts {
            println!("{}: {}", name, phone);
        }
    }
}

// 연락처를 삭제하는 함수
fn delete_contact(contacts: &mut HashMap<String, String>) {
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

// 파일에서 연락처를 로드하는 함수
fn load_contacts() -> HashMap<String, String> {
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
// 연습
// 연락처를 파일에 저장하는 함수
fn save_contacts(contacts: &HashMap<String, String>) {
    let mut content = String::new();
    // 각 연락처를 "이름:전화번호\n" 형식으로 포맷합니다
    for (name, phone) in contacts {
        content.push_str(&format!("{}:{}\n", name, phone));
    }
    // 파일에 씁니다
    let _ = fs::write(CONTACTS_FILE, content);
}
 