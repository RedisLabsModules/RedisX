#[macro_use]
extern crate redis_module;

use redis_module::{Context, NextArg, RedisError, RedisResult};

use std::time::Duration;
use std::usize;

///
/// X.PREPEND <key> <value>
///
fn prepend(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 3 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let mut value = args.next_string()?;

    let redis_key = ctx.open_key_writable(&key);
    let val = redis_key.read()?.unwrap(); // read on writeable always returns Some
    value.push_str(&val);
    redis_key.write(&value)?;

    ctx.replicate_verbatim();

    Ok(value.len().into())
}

///
/// X.GETSETEX <key> <value> <seconds>
///
fn getsetex(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 4 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let value = args.next_string()?;
    let seconds = args.next_u64()?;

    let redis_key = ctx.open_key_writable(&key);
    let res = if redis_key.is_empty() {
        ().into()
    } else {
        redis_key.read()?.unwrap().into() // read on writeable always returns Some
    };

    redis_key.write(&value)?;
    redis_key.set_expire(Duration::from_secs(seconds))?;

    ctx.replicate_verbatim();

    Ok(res)
}

///
/// X.GETEX <key> <seconds>
///
fn getex(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 3 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let seconds = args.next_u64()?;

    let redis_key = ctx.open_key_writable(&key);

    let res = if redis_key.is_empty() {
        ().into()
    } else {
        redis_key.set_expire(Duration::from_secs(seconds))?;
        ctx.replicate_verbatim();
        redis_key.read()?.unwrap().into() // read on writeable always returns Some
    };

    Ok(res)
}

///
/// X.GETDEL <key>
///
fn getdel(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let redis_key = ctx.open_key_writable(&key);
    let res = if redis_key.is_empty() {
        ().into()
    } else {
        let v = redis_key.read()?.unwrap(); // read on writeable always returns Some
        redis_key.delete()?;
        ctx.replicate_verbatim();
        v.into()
    };

    Ok(res)
}

///
/// X.INCRBYEX <key> <increment> <seconds>
///
fn incrbyex(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 4 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let increment = args.next_string()?;
    let seconds = args.next_u64()?;

    let res = ctx.call("INCRBY", &[&key, &increment])?;
    let redis_key = ctx.open_key_writable(&key);
    redis_key.set_expire(Duration::from_secs(seconds))?;
    
    ctx.replicate_verbatim();

    Ok(res)
}

///
/// X.HAPPEND <key> <field> <value>
///
fn happend(ctx: &Context, args: Vec<String>) -> RedisResult {
    if args.len() > 4 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let field = args.next_string()?;
    let value = args.next_string()?;

    let redis_key = ctx.open_key_writable(&key);
    let curr_value = redis_key.hash_get(&field)?;
    let res = if let Some(mut val) = curr_value {
        val.append(&value);
        val
    } else {
        ctx.create_string(&value)
    };
    let len = res.len();
    redis_key.hash_set(&field, res);

    ctx.replicate_verbatim();

    Ok(len.into())
}

redis_module! {
    name: "redisx",
    version: 999999,
    data_types: [],
    commands: [
        ["x.prepend", prepend, "write deny-oom"],
        ["x.getsetex", getsetex, "write deny-oom"],
        ["x.getex", getex, "write"],
        ["x.getdel", getdel, "write"],
        ["x.incrbyex", incrbyex, "write"],
        ["x.happend", happend, "write"],
    ],
}
