# Foundation Layer: Java to Rust Mapping Reference

## Overview

This document provides a detailed mapping of the Foundation layer components from Java Logisim-Evolution to Rust Logisim-RUST, showing the exact correspondence between classes, methods, and patterns.

## Utility Classes Mapping

### StringUtil & StringGetter

| Java | Rust | Notes |
|------|------|-------|
| `StringUtil.java` | `string_util.rs` | Static methods → associated functions |
| `StringGetter.java` | `StringGetter` trait | Interface → trait |
| `StringUtil.toHexString(int, int)` | `StringUtil::to_hex_string(u32, usize)` | Type-safe integer conversion |
| `StringUtil.isNullOrEmpty(String)` | `StringUtil::is_null_or_empty(Option<&str>)` | Null safety with Option |
| `StringUtil.resizeString(String, int)` | `StringUtil::resize_string(String, usize)` | Immutable string operations |

**Key Changes:**
- Null strings represented as `Option<String>` instead of nullable `String`
- Integer types explicitly sized (`u32`, `usize` instead of `int`)
- Immutable operations return new strings instead of modifying in-place

```java
// Java
String hex = StringUtil.toHexString(255, 2); // "FF"
if (StringUtil.isNullOrEmpty(str)) { ... }
```

```rust
// Rust
let hex = StringUtil::to_hex_string(255, 2); // "FF"
if StringUtil::is_null_or_empty(str.as_deref()) { ... }
```

### CollectionUtil

| Java | Rust | Notes |
|------|------|-------|
| `CollectionUtil.java` | `collection_util.rs` | Static utility class → module |
| `ArrayList<T>` | `Vec<T>` | Dynamic arrays |
| `HashMap<K,V>` | `HashMap<K,V>` | Hash maps (same interface) |
| `HashSet<T>` | `HashSet<T>` | Hash sets (same interface) |
| `isNullOrEmpty(Collection)` | `is_null_or_empty(Option<&Vec<T>>)` | Null safety |
| `unionOf(Set, Set)` | `union_set(HashSet<T>, HashSet<T>)` | Ownership semantics |

**Key Changes:**
- Collections cannot be null, use `Option<Collection<T>>` for optional collections
- Union operations consume owned collections (can be optimized with references)
- Generic type parameters are explicit and checked at compile time

```java
// Java
Set<String> result = CollectionUtil.unionOf(set1, set2);
if (CollectionUtil.isNullOrEmpty(list)) { ... }
```

```rust
// Rust
let result = CollectionUtil::union_set(set1, set2);
if CollectionUtil::is_null_or_empty(list.as_ref()) { ... }
```

### Cache

| Java | Rust | Notes |
|------|------|-------|
| `Cache.java` | `cache.rs` | Generic class → generic struct |
| `Cache<T>` | `Cache<T>` where `T: Clone + PartialEq + Hash` | Trait bounds explicit |
| `get(int)` | `get_by_hash(usize)` | Hash codes are usize |
| `put(T)` | `get_or_insert(T)` | Immutable caching pattern |
| `clear()` | `clear()` | Same interface |

**Key Changes:**
- Explicit trait bounds for cached types
- Thread safety built-in with `Send + Sync`
- Memory management automatic (no manual cleanup needed)

```java
// Java
Cache<String> cache = new Cache<>();
String result = cache.get(42);
cache.put("value");
```

```rust
// Rust
let mut cache = Cache::<String>::new();
let result = cache.get_by_hash(42);
let cached = cache.get_or_insert("value".to_string());
```

### FileUtil

| Java | Rust | Notes |
|------|------|-------|
| `FileUtil.java` | `file_util.rs` | Static methods → associated functions |
| `File` | `PathBuf` / `Path` | Rust's path handling |
| `IOException` | `Result<T, io::Error>` | Error handling with Result |
| `createTempFile()` | `create_temp_file()` | Same functionality |
| `getBytes(File)` | `get_bytes<P: AsRef<Path>>()` | Generic path handling |

**Key Changes:**
- All I/O operations return `Result` for error handling
- Paths are strongly typed (`Path` vs `String`)
- Cross-platform compatibility built-in

```java
// Java
try {
    byte[] data = FileUtil.getBytes(file);
} catch (IOException e) { ... }
```

```rust
// Rust
match FileUtil::get_bytes(&path) {
    Ok(data) => { ... },
    Err(e) => { ... }
}
```

### LocaleManager

| Java | Rust | Notes |
|------|------|-------|
| `LocaleManager.java` | `locale_manager.rs` | Singleton → global state |
| `LocaleListener.java` | Callback closures | Interface → closures |
| `getLocale()` | `get_locale()` | Same interface |
| `getString(String)` | `get_string(&str)` | String references |
| `addLocaleListener()` | Event system with closures | Type-safe callbacks |

**Key Changes:**
- Global state managed safely with `Once` initialization
- Listeners use closures instead of interface implementations
- String keys are string references for efficiency

```java
// Java
LocaleManager.addLocaleListener(new LocaleListener() {
    public void localeChanged() { ... }
});
String text = LocaleManager.getString("key");
```

```rust
// Rust
LocaleManager::add_locale_listener(|| { ... });
let text = LocaleManager::get_string("key");
```

## Data Structures Mapping

### Direction

| Java | Rust | Notes |
|------|------|-------|
| `Direction.java` | `direction.rs` | Class → enum |
| `Direction.EAST` | `Direction::East` | Constants → enum variants |
| `getLeft()` | `get_left()` | Method → method |
| `toRadians()` | `to_radians()` | Same interface |
| `parse(String)` | `parse(&str)` → `Result<Direction, String>` | Error handling |

**Key Changes:**
- Enum instead of class with constants provides better type safety
- Pattern matching available for control flow
- Parsing returns Result for error handling

```java
// Java
Direction dir = Direction.EAST;
Direction left = dir.getLeft();
int degrees = dir.toDegrees();
```

```rust
// Rust
let dir = Direction::East;
let left = dir.get_left();
let degrees = dir.to_degrees();
```

### Location

| Java | Rust | Notes |
|------|------|-------|
| `Location.java` | `location.rs` | Class → struct |
| `Location(int, int)` | `Location::new(i32, i32)` | Constructor → associated function |
| `getX()` | `get_x()` | Same interface |
| `translate(int, int)` | `translate(i32, i32)` | Returns new Location (immutable) |
| `manhattanDistanceTo()` | `manhattan_distance_to()` | Same functionality |

**Key Changes:**
- Struct with Copy semantics for performance
- All operations immutable (return new instances)
- Explicit integer types

```java
// Java
Location loc = new Location(10, 20);
Location moved = loc.translate(5, 0);
int dist = loc.manhattanDistanceTo(other);
```

```rust
// Rust
let loc = Location::new(10, 20);
let moved = loc.translate(5, 0);
let dist = loc.manhattan_distance_to_location(other);
```

### Bounds

| Java | Rust | Notes |
|------|------|-------|
| `Bounds.java` | `bounds.rs` | Class → struct |
| `Bounds.create()` | `Bounds::create()` | Static method → associated function |
| `contains(int, int)` | `contains(i32, i32)` | Same interface |
| `add(Bounds)` | `add_bounds(Bounds)` | Immutable operations |
| `intersects()` | `intersect()` | Returns new Bounds |

**Key Changes:**
- Immutable operations return new Bounds
- Copy semantics for performance
- Built-in Display trait for debugging

```java
// Java
Bounds bounds = Bounds.create(0, 0, 100, 50);
Bounds expanded = bounds.add(other);
boolean contains = bounds.contains(x, y);
```

```rust
// Rust
let bounds = Bounds::create(0, 0, 100, 50);
let expanded = bounds.add_bounds(other);
let contains = bounds.contains(x, y);
```

### BitWidth

| Java | Rust | Notes |
|------|------|-------|
| `BitWidth.java` | `bit_width.rs` | Class → struct |
| `BitWidth.create(int)` | `BitWidth::create(u32)` → `Result<BitWidth, String>` | Error handling |
| `getWidth()` | `get_width()` | Same interface |
| `getMask()` | `get_mask()` | Returns u64 mask |
| Compatibility with `BusWidth` | `From<BusWidth>` / `Into<BusWidth>` | Trait-based conversion |

**Key Changes:**
- Width validation returns Result instead of throwing exceptions
- Explicit unsigned integer types
- Trait-based conversion with existing types

```java
// Java
BitWidth width = BitWidth.create(8);
int w = width.getWidth();
long mask = width.getMask();
```

```rust
// Rust
let width = BitWidth::create(8)?;
let w = width.get_width();
let mask = width.get_mask();
```

### Attribute System

| Java | Rust | Notes |
|------|------|-------|
| `Attribute.java` | `Attribute<T>` | Generic class → generic struct |
| `AttributeSet.java` | `AttributeSet` | Class → struct |
| `AttributeOption.java` | `AttributeOption<T>` | Generic class → generic struct |
| `getValue(Attribute<V>)` | `get_value<T>(&Attribute<T>)` → `Option<&T>` | Type-safe retrieval |
| `setValue(Attribute<V>, V)` | `set_value<T>(&Attribute<T>, T)` → `Result<(), String>` | Error handling |

**Key Changes:**
- Compile-time type safety for attribute values
- No runtime type checking needed
- Trait-based value conversion and validation

```java
// Java
AttributeSet attrs = new AttributeSet();
Attribute<Integer> attr = new Attribute<>("width");
attrs.setValue(attr, 8);
Integer value = attrs.getValue(attr);
```

```rust
// Rust
let mut attrs = AttributeSet::new();
let attr = Attribute::<u32>::new("width".to_string());
attrs.set_value(&attr, 8)?;
let value = attrs.get_value(&attr);
```

## Error Handling Patterns

### Java Exceptions → Rust Results

| Java Pattern | Rust Pattern | Notes |
|--------------|--------------|-------|
| `throws IOException` | `Result<T, io::Error>` | Explicit error handling |
| `IllegalArgumentException` | `Result<T, String>` | Custom error messages |
| `NullPointerException` | Impossible | Compile-time null safety |
| `try-catch` | `match` or `?` operator | Pattern matching or propagation |

### Java Null → Rust Option

| Java Pattern | Rust Pattern | Notes |
|--------------|--------------|-------|
| `String str = null` | `let str: Option<String> = None` | Explicit null representation |
| `if (str != null)` | `if let Some(s) = str` | Pattern matching |
| `str.length()` | `str.as_ref().map(|s| s.len())` | Safe operations |

## Performance Considerations

### Memory Management

| Java | Rust | Performance Impact |
|------|------|--------------------|
| Garbage Collection | Ownership System | Predictable performance |
| Object allocation | Stack allocation | Reduced memory pressure |
| Reference counting | Move semantics | Zero-cost abstractions |

### Type System

| Java | Rust | Performance Impact |
|------|------|--------------------|
| Runtime type checking | Compile-time verification | Zero runtime overhead |
| Boxing primitives | Native value types | No allocation overhead |
| Virtual method calls | Static dispatch (default) | Inlining opportunities |

This mapping guide ensures that developers familiar with the Java implementation can quickly understand and work with the Rust Foundation layer while leveraging Rust's safety and performance benefits.