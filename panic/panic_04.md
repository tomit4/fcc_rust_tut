## unwinding and abort

By default, when a `panic` occurs, the program starts <em>unwinding</em>, which
means Rust walks back up the stack and cleans up the data from each function it encounters.

But this walk back and clean up is a lot of work. The alternative is to
immediately abort the program without cleaning up.

If in your project you need to make the resulting binary as small as possible,
you can switch from unwinding to aborting by adding below content to `Cargo.toml`:

```toml
[profile.release]
panic = 'abort'
```
