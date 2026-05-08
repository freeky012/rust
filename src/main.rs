use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Clone)]
pub struct Contact {
    pub phone: String,
    pub address: String,
}

mod add_contact;
mod view_contact;
mod list_contacts;
mod delete_contact;
mod load_contacts;
mod save_contacts;

use add_contact::add_contact;
use view_contact::view_contact;
use list_contacts::list_contacts;           
use delete_contact::delete_contact;
use load_contacts::load_contacts;
use save_contacts::save_contacts;





fn main() {
    // 파일에서 기존 연락처를 로드합니다
    let mut contacts: HashMap<String, Contact> = load_contacts();
    
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


