# hello-macro

Check out the `hello-tokio` example for a macro that:
- runs the macro before `tokio` runtime is spawned
- runs the body of `main` inside the `tokio` runtime

You can compare the output of `cargo expand` for `hello-async` and `hello-tokio`.