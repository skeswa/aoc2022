use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
  /// Mapping from certain characters ([a-zA-Z]) to priority integers.
  static ref PRIORITY_BY_ITEM_TYPE: HashMap<char, u8> =
      index_priorities_by_item_type();
}

/// Returns the priority of the specified `item_type` character.
pub(crate) fn priority_of(item_type: &char) -> Option<&u8> {
    PRIORITY_BY_ITEM_TYPE.get(item_type)
}

/// Generates [PRIORITY_BY_ITEM_TYPE].
fn index_priorities_by_item_type() -> HashMap<char, u8> {
    let mut priority_by_item_type = HashMap::new();

    let mut priority = 1;

    for code_point in b'a'..=b'z' {
        let item_type = char::from(code_point);

        priority_by_item_type.insert(item_type, priority);
        priority += 1;
    }

    for code_point in b'A'..=b'Z' {
        let item_type = char::from(code_point);

        priority_by_item_type.insert(item_type, priority);
        priority += 1;
    }

    priority_by_item_type
}
