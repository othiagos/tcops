use crate::common::instance::HasId;
use std::io::{Error, ErrorKind};


pub fn validate_section_data_id(section: &str, id: usize, last_id: isize) -> Result<(), Error> {
    if id as isize != last_id + 1 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("{} id {} need be sequential", section, id),
        ));
    }

    Ok(())
}

pub fn validate_item_id<T>(container: &[T], item_id: usize) -> Result<(), Error>
where
    T: HasId,
{
    let item = container.get(item_id).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Integrity error: Item ID {} does not exist.", item_id),
        )
    })?;

    if item.id() != item_id {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Integrity error: Item ID {} does not exist.", item_id),
        ));
    }

    Ok(())
}