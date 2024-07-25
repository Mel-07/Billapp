// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::io;
use std::collections::*;
#[derive(Debug)]
enum Menu {
    AddBill,
    ViewBill,
    RemoveBill,
    EditBill,
}
#[derive(Debug,Clone)]
 struct Bill {
    name:String,
    amount:f64
}


fn get_user_input()->Option<String>{
    let mut buffer = String::new();
   if let Ok(_) = io::stdin().read_line(&mut buffer){
    Some(buffer.trim().to_string())
   }else{
    None
   }

}
fn check_menu_list(list:&str,store_bills: &mut HashMap<String,Bill>){

    match list {
        "1" => selected_menu(Menu::AddBill, store_bills),
        "2" => selected_menu(Menu::ViewBill, store_bills),
        "3" => selected_menu(Menu::RemoveBill, store_bills),
        "4" => selected_menu(Menu::EditBill, store_bills),
        _ => println!("{:?} is not a Valid menu",list)
    }
}

fn selected_menu(menu_type:Menu, store_bills:&mut HashMap<String,Bill>){

    use working_on_bills::*;


    match menu_type {
        Menu::AddBill => add_bill(store_bills),
        Menu::ViewBill => view_bills(store_bills),
        Menu::RemoveBill => remove_bill(store_bills),
        Menu::EditBill => edit_bills(store_bills)
    }
}

fn show_menu(){
    let menu:Vec<&str> = vec!["Select Menu From one to four"," ","==Bill Manager==", "1. Add Bill","2. View Bills","3. Remove Bill","4. Edit Bill" ," ","Enter Selection: "];
for set_menu in menu {
        println!("{}",set_menu);
}
}

fn main() {
    let mut store_bills:HashMap<String,Bill> =HashMap::new();
    
loop{
    show_menu();
    match get_user_input(){

        Some(menu_match) => check_menu_list(&menu_match,&mut store_bills) ,
        None => ()
    }
}
}
/* functions for working on bills
These functions includes
1. add_bill -> for adding new bills,
2. view bills -> for view_bills for viewing all bills,
3. remove_bills -> for removing_bill (which can be based on keys from HashMap),
4. edit bills  -> for editing the bill by selecting it based on key and get getting a ##mutable value##
*/
// for creating keys for hashmap function

mod working_on_bills{
    use crate::*;
    pub fn add_bill(bills: &mut HashMap<String,Bill>) {
        loop {
            println!("Item:");
            println!(" ");
            let name = match get_user_input(){
                Some (name)=>{
                    if name == "" || name == " "{
                        return
                    }
                    else{
                        name
                    }
                },
                None => return
            };
            match check_if_bill_name_exist(bills,&name) {
                Some(v) =>{
                    if v{
                        println!("");
                        println!("Name Already Exist in list");
                        return
                    }
                },
                None => ()
            }
    
            println!("Amount");
            println!(" ");
            let amount: f64 = match get_user_input().expect("Failed to read input").trim().parse() {
                Ok(amount) => amount,
                Err(_) => {
                    println!("Amount must be a number");
                    return;
                }
            };
    
            bills.insert(name.clone(), Bill { name, amount });
            println!(" ");
            for bill in bills {

                println!("{} {}" , bill.1.name, bill.1.amount);
            }
    
            break;
        }
    }

    pub fn view_bills(view_bills:&mut HashMap<String,Bill>){

        println!("== Bills ==");
        println!(" ");

        for bill in view_bills {
            println!("{}  {}",  bill.1.name, bill.1.amount);
        }
    }
    pub fn remove_bill(remove_bill: &mut HashMap<String,Bill>){

        if remove_bill.len() == 0{
            println!("No Bill Available");
            return
        }
        println!(" ");
        println!("Enter Bill Name to remove");

        match get_user_input(){
            Some(key_to_remove_item) => {
                match remove_bill.remove(&key_to_remove_item){
                    Some(value) => println!("{:?},{:?} is removed",value.name,value.amount),
                    None => println!("Not in the list") 
                }
            },
            None => return
        }
    }
    pub fn edit_bills(bill:&mut HashMap<String,Bill>){
        /*after clicking on number 4 it should show the list of item in order to 
        allow the user pick the item to edit */
        if bill.is_empty() {
            println!("No List to Edit");
        }else{
            println!("S/N Name  Amount");
            for (_,value) in bill.iter(){

                println!("{} {}", value.name,value.amount);
            }
            println!(" ");

            let gotten_value:Option<String> =loop {
                println!("Enter Name of Item to Edit");
                let menu_number = match  get_user_input() {
                    Some(va) => {
                        Some(va)
                    },
                    None =>None 
                };
                break menu_number;
            };

            match gotten_value {
                Some(item_name) =>{
                    if  let Some(gotten_bill) =  bill.get_mut(&item_name){
                        println!("Select Which Field you what to Edit");
                        println!("1. Name");
                        println!("2. Amount");
                        match get_user_input(){
                            Some (field_to_edit) => {
                                match field_to_edit.as_str() {
                                    "1" =>{
                                        match get_user_input(){
                                            Some(input) => {
                                                gotten_bill.name = input;
                                                
                                            },
                                            None => println!("nothing added")
                                        }
                                    },
                                    "2" => {
                                        match get_user_input(){
                                            Some(input) => {
                                                let new_amount:Result<f64,_> = input.parse();
                                                if let Ok(new_amo) = new_amount{
                                                    gotten_bill.amount = new_amo;
                                                }
                                                else {
                                                    println!("not a number");
                                                }
                                                
                                            },
                                            None => println!("nothing added")
                                        }
                                    },
                                    _=> println!("Not an OPtion")
                                }
                            },
                            None => println!("no field sleceted")
                        }
                    }else{
                        println!("no such bill");
                    }
                },
                None => println!("{:?}", "Not a number")

            }

        }

    }

    fn check_if_bill_name_exist(bill:&mut HashMap<String,Bill>, name:&String)->Option<bool>{

        for (_k,v) in bill.iter(){
            if &v.name == name{
                return Some(true)
            }
            else{
               return Some(false)
            }
        }
        None
    }
}