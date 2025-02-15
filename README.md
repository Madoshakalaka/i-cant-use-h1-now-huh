# Wild

```
cargo +nightly check
```


```
    Checking trunk-template v0.1.0 (/home/maa/Projects/i-cant-use-h1-now-huh)
warning: The tag 'h1' is not matching its normalized form 'h1'. If you want to keep this form, change this to a dynamic tag `@{"h1"}`.
 --> src/app.rs:6:10
  |
6 |         <h1>{ "Help!" }</h1>
  |          ^^

warning: The tag 'h2' is not matching its normalized form 'h2'. If you want to keep this form, change this to a dynamic tag `@{"h2"}`.
 --> src/app.rs:7:10
  |
7 |         <h2>{ "What even!" }</h2>
  |          ^^

warning: `trunk-template` (bin "trunk-template") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

# Hmmm

`Cargo.toml` has two `yew-html-ext`. One new and one old.

The new triggers the warning with nightly. The old doesn't. 

Attached in [src/app.rs](./src/app.rs) also are two possible regressions where braceless style has become invalid.
