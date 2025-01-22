
use clap::Subcommand;
use tmflib::{
    HasId,
    HasName,
    HasDescription,
};

pub mod tmf620;
pub mod tmf622;
pub mod tmf629;
pub mod tmf632;
pub mod tmf633;
pub mod tmf648;

#[derive(Clone, Subcommand, Debug)]
pub enum TMFOperation {
    List,
    Get {
        id : String,
    },
    Create,
    Update,
    Delete
}

pub fn iterate_name<T : HasId + HasName>(items : &Vec<T>) {
    items.iter().for_each(|i| {
        println!("Item: [{}] {} [{}]",T::get_class(),i.get_name(),i.get_id());
    });
}

pub fn iterate_desc<T : HasId + HasDescription>(items : &Vec<T>) {
    items.iter().for_each(|i| {
        println!("Item: [{}] {} [{}]",T::get_class(),i.get_description(),i.get_id());
    });
}

pub fn display_id<T: HasId>(item : &T) {
    println!("Id:\t{}",item.get_id());
    println!("Href:\t{}",item.get_href());    
}

pub fn display_name<T: HasId + HasName>(item : &T) {
    display_id(item);
    println!("Name:\t{}",item.get_name());
    
}

pub fn display_desc<T : HasId + HasDescription>(item : &T) {
    display_id(item);
    println!("Desc:\t{}",item.get_description());
}

pub fn display_opt(label : &str, field : &Option<String>) {
    match field {
        Some(v) => println!("{}:\t{}",label,v),
        None => println!("{}:\tNot Set",label),
    }
}