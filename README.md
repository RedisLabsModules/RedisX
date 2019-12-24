[![GitHub issues](https://img.shields.io/github/release/RedisLabsModules/RedisX.svg)](https://github.com/RedisLabsModules/RedisX/releases/latest)
[![CircleCI](https://circleci.com/gh/RedisLabsModules/RedisX/tree/master.svg?style=svg)](https://circleci.com/gh/RedisLabsModules/RedisX/tree/master)

# RedisX
Extension modules to Redis' native data types and commands


# Getting Started

## Build

```bash
git clone https://github.com/RedisLabsModules/RedisX.git
cd RedisX
cargo build --release
```

## Run 
```
redis-server --loadmodule ./target/release/libredisx.so
```

# Commands

## X.GETSETEX key value seconds
`Time complexity: O(1)`

Atomically sets key to value and returns the old value stored at key
and set key to timeout after a given number of seconds.
Returns an error when key exists but does not hold a string value.

This command is equivalent to executing the following commands:

```
GETSET mykey value
EXPIRE mykey seconds
```

### Return value
[Bulk string reply](https://redis.io/topics/protocol#bulk-string-reply): the value of key, or nil when key does not exist.



## X.GETEX key seconds
`Time complexity: O(1)`

Get the value of key and set key to timeout after a given number of seconds.
If the key does not exist the special value nil is returned. 
An error is returned if the value stored at key is not a string, because GETEX only handles string values.

This command is equivalent to executing the following commands:

```
GET mykey
EXPIRE mykey seconds
```

### Return value
[Bulk string reply](https://redis.io/topics/protocol#bulk-string-reply): the value of key, or nil when key does not exist.



## X.GETDEL key
`Time complexity: O(1)`

Removes the specified keys and returns the old value stored at key.
If the key does not exist the special value nil is returned.
An error is returned if the value stored at key is not a string, because GETDEL only handles string values.

This command is equivalent to executing the following commands:

```
GET mykey
DEL mykey
```

### Return value
[Bulk string reply](https://redis.io/topics/protocol#bulk-string-reply): the value of key, or nil when key does not exist.



## X.PREPEND key value
`Time complexity: O(1). The amortized time complexity is O(1) assuming the appended value is small and the already present value is of any size`

If key already exists and is a string, this command prepend the value at the begin of the string. 
If key does not exist it is created and set as an empty string, so PREPEND will be similar to SET in this special case.

### Examples
```
redis>  EXISTS mykey
(integer) 0
redis>  X.PREPEND mykey "World"
(integer) 5
redis>  X.PREPEND mykey "Hello "
(integer) 11
redis>  GET mykey
"Hello World"
redis> 
```
### Return value
[Integer reply](https://redis.io/topics/protocol#integer-reply): the length of the string after the prepend operation.



## X.INCRBYEX <key> <increment> <seconds> 
`Time complexity: O(1).`

Increments the number stored at key by increment and set key to timeout after a given number of seconds. If the key does not exist, it is set to 0 before performing the operation. An error is returned if the key contains a value of the wrong type or contains a string that can not be represented as integer. This operation is limited to 64 bit signed integers.

This command is equivalent to executing the following commands:

```
INCRBY mykey value
EXPIRE mykey seconds
```

### Examples
```
redis>  SET mykey "10"
"OK"
redis>  X. INCRBYEX mykey 5 
(integer) 15
```
### Return value
[Integer reply](https://redis.io/topics/protocol#integer-reply): the length of the string after the prepend operation.


## X.HAPPEND key field value
`Time complexity: O(1). The amortized time complexity is O(1) assuming the appended value is small and the already present value is of any size`

If key and field already exist, this command appends the value at the end of the field. 
If key does not exist, a new key holding a hash is created. 
If field does not exist it is created and set as an empty string, so X.HAPPEND will be similar to HSET in this special case.

### Examples
```
redis>  EXISTS mykey
(integer) 0
redis>  X.HAPPEND mykey myfield "World"
(integer) 5
redis>  X.HAPPEND mykey myfield "Hello "
(integer) 11
redis>  HGET mykey myfield
"Hello World"
redis> 
```
### Return value
[Integer reply](https://redis.io/topics/protocol#integer-reply): the length of the string after the prepend operation.



# Remark 
Based on https://github.com/RedisLabsModules/redex
