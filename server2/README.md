# AeroRust

A WIP oxidation of Aero.

## Motivation

- Most AeroServer bugs that I've encountered have been memory-related:
  use-after-free, observers failing to clean up, misuse of pointers &
  reference-like types, unsafe concurrent access.
  Rust prevents this category of bugs at compile-time.
- Our code is not thread safe and crashes if we try to enable multiple
  web handler threads. This means that long-running web requests block
  the entire process. Rust makes it easy to safely introduce concurrency
  and has many easy-to-use web frameworks.
- Rust has a rich library of 3rd-party crates. It's very easy to pull in
  high-quality code for common use-cases. This doesn't mean that we should
  add dependencies all over the place, but it is useful for library code that
  we shouldn't maintain ourselves - networking, parsing, date handling, data
  structures, logging framework, web/http framework, DB adapters, process spawning,
  cryptography, serializers.
- We can automatically derive JSON & YAML serializers from rust types with `serde`.
  There also crates that generate JSON-schema and TypeScript definitions. **We can
  get rid of our SchemaDSL project**.
- Using 3rd-party crates also potentially eliminates other common bugs. Many
  of our bugs have been in library code that we've written ourselves. Some examples
  include: resolving domains that map to IPv6 addresses (still doesn't seem to be
  fixed). FIX: Find other motivating bugs.
- The migration can be done incrementally. We can keep the majority of our C++ code
  and replace/migrate pieces as needed. In most cases, we can call into C++ code from
  rust or rust code from C++ without performance loss, due to link-time optimization.
- Makes it simpler to create a plugable architecture with Web Assembly. Wasmtime's main
  bindings are in Rust, while the C++ binding is a wrapper around the C api.
- Can be simpler to support other architectures. Rust has a strong cross-compilation
  story with many supported targets, and doesn't have the same implicit UB issues that C++
  does - in C++ migrating from x86_64 to aarch64 is often difficult because the memory model
  is different. It's easy in C++ to depend on implicit behavior of the x86 model even though
  it is technically UB.

## Roadmap

### Phase 1

Handle command-line parsing, config file opening, log files. Defer event loop to C++ Aero

1. Replace the main function with Rust. Rust handles parsing command-line options,
   initializing the logger, and reading the config file from the filesystem.
2. Rust constructs an instance of CApplication using `cxx`. Value will be stored in
   C++ heap, and must exist for entirety of main function.
3. Shims are written around the command-line options so that they
   communicate with the rust versions. This can be done by changing the exposed
   CCommandlineParams object to a limited `ICommandlineParams` interface that supports
   at minimum `GetFlagStr`, `GetFlagBool`, `GetFlagInt`, `HasFlag`, and `IsChanged`.
   We don't allow mutating the options, so in Rust we can store the options as a
   global variable with static lifetime:
   `static mut g_CmdlineParams: Option<CommandlineOpts> = None;`.
   The only time we mutate this variable should be at the start of the main function,
   and Rust requires an unsafe block to mutate it.
   Rust provides functions which can be used to implement the `ICommandlineParams` interface.
4. Do the same for DocDebug/CSysLog. In rust, logging is just a macro call which uses
   a global, so we can provide a function which takes a C++ string, module, and severity/color
   and passes it to these macros. We'll probably change the log-file format while this happens,
   html is not good due to escaping issues. The nice thing is that application code will only
   use functions provided by the `log` crate, so we're free to switch out the implementation
   if needed. Right now I'm planning on using `flexi_logger`.
5. Rust calls CApplication's Run function, starting the C++ asio loop. The config
   file text and file name is passed in.

### Phase 2

Control event loop in Rust Aero with tokio. We poll asio at the end of the loop
to allow C++ async code to run. Use `asio::io_context::run_until` set to 200ms before
the next loop should start. Make sure to remove the work_guard from C++ Aero, and restart
the io_context after each call to `run_until`.

What about transports? It appears that they were always controlled by asio and triggered by
IOnTick methods.

### Phase 3

Start migrating networking code. HTTP web server can be moved directly to Rust,
since it's mostly just serving from the filesystem. WSAPI handlers always use the
CWSApiLink::Send method, which takes raw bytes and a size. We can probably replace
CWSApiLink with a implementation which communicates with a Rust websocket api.

At this point we must make sure tokio is still running in single-threaded mode. Allowing
the C++ WSApi handlers to access C++ singletons from other threads can introduce those
same crashes that we saw when increasing ASIO's thread count.

To work around this, we can introduce locks for global singletons. Locking in C++ code
will mean changes to the entire codebase, while locking in Rust means that we have to lock
before each call to a WSApi handler, pass in the singleton instance (likely just CApplication
to start), and then unlock once the handler returns. This will probably cancel out any of the
multithreading benefit since only one handler can run at a time.

Convert other networking code (TCP, UDP acceptors, Serial, MQTT, etc.). The process should
be similar - keep the existing interface and implement it using Rust FFI calls.

Now we should be able to remove ASIO, which has been replaced with Rust's tokio. Tokio
is still single threaded, and this won't change until we can move all of our global objects
to be controlled by Rust instead of C++.

### Phase 4

Add a C++ -> Rust api that allows accessing our Variables. Since Variables may be
arbitrarily mutated, this can't be safe in Rust world. We don't allow holding on to
references, just asking for a particular variable value, quality, etc. given the opaque
Variable object. This should ensure that C++ callers never mutate a value held by Rust
code, but doesn't provide any safety benefits to existing C++ consumers.

Add a WebAssembly plugin system for Calculation-type plugins. To simplify things, we'll
run calculation plugins after all of our regular Calculations. Calculation plugins can
depend on variables produced by each other and built-in Calculations, but built-in
Calculations can't use variables created by plugins. Execution order will be determined
by a topological sort of the dependency DAG.

At this point, all of our networking code should be in Rust, so we might be able to
support Sensor-type plugins as well (with limited functionality, since right now all
they can access are Variables and the network).
