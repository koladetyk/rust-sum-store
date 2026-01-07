# SumStore – Rust Take-Home Exercise

## Overview
This project implements a simple Rust library that stores the sum of two integers with
role-based access control and unit tests.

## How to Run the Tests

The functionality is exercised through unit tests in `src/lib.rs`.

---

From the project root:

```bash
cargo test
```

To see test log output:

```bash
cargo test -- --nocapture
```

## Assumptions & Design Decisions

1. **Explicit unset state**: The stored sum is represented as `Option<i32>` to distinguish between an unset state (`None`) and a legitimate sum of zero (`Some(0)`). This provides clearer semantics than using a default value of `0`.

2. **Setter replaces value**: Each successful call to `set_sum` overwrites the previously stored sum rather than accumulating values.

3. **Role passed per call**: Authorization is simulated by passing a `Role` parameter to the setter rather than storing identity state with the struct. This simulates a request-based authentication system.

4. **Failed authorization has no side effects**: Unauthorized setter calls do not modify internal state. 

5. **Simple access control**: Uses `if role != Role::Admin` for clarity and readability, appropriate for a binary permission check.