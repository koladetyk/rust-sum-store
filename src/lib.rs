#[derive(Debug, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, PartialEq)]
pub enum SumError {
    NotAuthorized,
}

#[derive(Debug, Default)]
pub struct SumStore {
    stored_sum: Option<i32>,
}

impl SumStore {
    /// Creates a new SumStore with no sum set
    pub fn new() -> Self {
        Self { stored_sum: None }
    }

    /// Setter function: only callable by admins
    pub fn set_sum(&mut self, a: i32, b: i32, role: Role) -> Result<(), SumError> {
        if role != Role::Admin {
            return Err(SumError::NotAuthorized);
        }

        self.stored_sum = Some(a + b);
        Ok(())
    }

    /// Getter function: public and accessible by anyone
    #[must_use]
    pub fn get_sum(&self) -> Option<i32> {
        self.stored_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to print Option<i32> nicely in test output
    fn fmt_sum(sum: Option<i32>) -> String {
        match sum {
            Some(v) => v.to_string(),
            None => "<unset>".to_string(),
        }
    }

    #[test]
    fn admin_can_set_sum() {
        println!("TEST 1: admin_can_set_sum");

        let mut store = SumStore::new();
        println!("Initial sum: {}", fmt_sum(store.get_sum()));

        let result = store.set_sum(5, 7, Role::Admin);
        println!("Setter result: {:?}", result);

        assert!(result.is_ok());
        assert_eq!(store.get_sum(), Some(12));

        println!("Final sum: {}", fmt_sum(store.get_sum()));
    }

    #[test]
    fn non_admin_cannot_set_sum() {
        println!("TEST 2: non_admin_cannot_set_sum");

        let mut store = SumStore::new();
        let result = store.set_sum(5, 7, Role::User);

        println!("Setter result: {:?}", result);

        assert_eq!(result, Err(SumError::NotAuthorized));
        assert_eq!(store.get_sum(), None);

        println!("Sum after failed set: {}", fmt_sum(store.get_sum()));
    }

    #[test]
    fn getter_returns_correct_value() {
        println!("TEST 3: getter_returns_correct_value");

        let mut store = SumStore::new();
        store.set_sum(10, 15, Role::Admin).unwrap();

        let value = store.get_sum();
        println!("Retrieved sum: {}", fmt_sum(value));

        assert_eq!(value, Some(25));
    }

    #[test]
    fn getter_accessible_for_any_role() {
        println!("TEST 4: getter_accessible_for_any_role");

        let mut store = SumStore::new();
        store.set_sum(3, 4, Role::Admin).unwrap();

        let value = store.get_sum();
        println!("Getter accessed without role, value = {}", fmt_sum(value));

        assert_eq!(value, Some(7));
    }

    #[test]
    fn initial_sum_is_none() {
        println!("TEST 5: initial_sum_is_none");

        let store = SumStore::new();
        println!("Initial sum: {}", fmt_sum(store.get_sum()));

        assert_eq!(store.get_sum(), None);
    }

    #[test]
    fn multiple_admin_updates() {
        println!("TEST 6: multiple_admin_updates");

        let mut store = SumStore::new();

        store.set_sum(1, 2, Role::Admin).unwrap();
        println!("After first update: {}", fmt_sum(store.get_sum()));
        assert_eq!(store.get_sum(), Some(3));

        store.set_sum(10, 20, Role::Admin).unwrap();
        println!("After second update: {}", fmt_sum(store.get_sum()));
        assert_eq!(store.get_sum(), Some(30));
    }

    #[test]
    fn sum_unchanged_after_failed_set() {
        println!("TEST 7: sum_unchanged_after_failed_set");

        let mut store = SumStore::new();
        store.set_sum(5, 5, Role::Admin).unwrap();
        println!("Initial authorized sum: {}", fmt_sum(store.get_sum()));

        let result = store.set_sum(100, 100, Role::User);
        println!("Unauthorized attempt result: {:?}", result);

        assert_eq!(result, Err(SumError::NotAuthorized));
        assert_eq!(store.get_sum(), Some(10));

        println!("Sum after unauthorized attempt: {}", fmt_sum(store.get_sum()));
    }
}
