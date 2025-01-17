
use clap::Subcommand;
use tmflib::{
    HasId,
    HasName,
    HasDescription,
};

pub mod tmf620;

#[derive(Clone, Subcommand, Debug)]
pub enum TMFOperation {
    List,
    Get,
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