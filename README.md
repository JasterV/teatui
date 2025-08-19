# TeaTui

> Tea = The Elm Architecture

This is an experimental project that aims to build an Elm-like framework for TUI development in Rust on top of [Ratatui](https://github.com/ratatui/ratatui).

My main motivation to build this was to understand if I could structure a [Ratatui](https://github.com/ratatui/ratatui) application in an Elm-like fashion.

The provided templates from the Ratatui official sources write code in a very procedural way, abusing of mutability a bit too much for my taste. (That is, using `&mut` all over the place in function arguments).

I want to minimize the usage of mutability and build the most pure functional TUI apps that I can.

I feel that Elm has always been a great example to follow when it comes to building frontend in a pure-functional style, and so I will try to reproduce it in here.

## How does it work

The state of your application is represented by a single type called the Model.

The Model will be used by a `view` process to render a View.

A separate process will read events from the outside world and
send them to an `update` process.

The `update` process will take the model and an event and
return a new model, potentially also returning a side effect.

The updated model will be sent to the `view`, triggering a new render
based on the new state of the application.

If any side effects are returned from `update`, they will be processed
in a separate process.

If the process responsible for handling side effects wants to update
the state of the application, it will send a message to the `update` process.

The users of this framework only need to provide:

- An update function that given a model and a message return an `Update` instance.

- A view function that given a reference to the model, returns a `View`

- An effects function that given a reference to the model and an effect,
 might perform any side effects and optionally return a message to update the state of the application

### Examples

You can find a folder with example projects in the [examples](https://github.com/JasterV/teatui/tree/main/examples) folder.

## License

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
