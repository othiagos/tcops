use crate::common::instance::HasId;
use std::io::{Error, ErrorKind};


pub fn validate_section_data_id(section: &str, id: usize, last_id: isize) -> Result<(), Error> {

    if id > 0 && last_id + 1 == 0 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("The {} id must start with 0", section),
        ));
    }

    if id as isize != last_id + 1 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("{} id {} need be sequential", section, id),
        ));
    }

    Ok(())
} 

pub fn validate_item_id<T>(container_name: &str, container: &[T], item_id: usize) -> Result<(), Error>
where
    T: HasId,
{
    let item = container.get(item_id).ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Integrity error: {} ID {} does not exist.", container_name, item_id),
        )
    })?;

    if item.id() != item_id {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Integrity error: {} ID {} does not exist.", container_name, item_id),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyItem {
        id: usize,
    }

    impl HasId for DummyItem {
        fn id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn test_validate_section_data_id() {
        // First item starting at 0
        assert!(validate_section_data_id("Node", 0, -1).is_ok());

        // First item starting at 1 (should fail)
        let err = validate_section_data_id("Node", 1, -1);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("The Node id must start with 0"));

        // Sequential items
        assert!(validate_section_data_id("Node", 1, 0).is_ok());
        assert!(validate_section_data_id("Node", 2, 1).is_ok());

        // Non-sequential items
        let err = validate_section_data_id("Node", 3, 1);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Node id 3 need be sequential"));

        // Duplicate item
        let err = validate_section_data_id("Node", 1, 1);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Node id 1 need be sequential"));
    }

    #[test]
    fn test_validate_item_id() {
        let items = vec![DummyItem { id: 0 }, DummyItem { id: 1 }];

        // Valid item IDs
        assert!(validate_item_id("Node", &items, 0).is_ok());
        assert!(validate_item_id("Node", &items, 1).is_ok());

        // Out of bounds ID
        let err = validate_item_id("Node", &items, 2);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Integrity error: Node ID 2 does not exist."));

        // Item at index 0 but id is 99
        let mismatched = vec![DummyItem { id: 99 }];
        let err = validate_item_id("Node", &mismatched, 0);
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Integrity error: Node ID 0 does not exist."));
    }
}