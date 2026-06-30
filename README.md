# A part of Miyuki X Editor   

```rust
fn main() {
    let opt_system_time= <DateTime<Local> as CustomTime>::new();
    let system_time= match opt_system_time {
        Some(time) => time,
        None => panic!("[Err] can't get local time")
    };

    let string_time= <DateTime<Local> as CustomTime>::get_string_format(&system_time);
}
```