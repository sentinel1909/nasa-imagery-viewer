# NASA Imagery Viewer

This site is written in Rust using the Yew web framework. It produces a site which displays the image of the day, along with the associated image title and explanation, by fetching from tHe NASA APOD (Astronomy Photo of the Day) API.

## Getting Started

The API key for the NASA APOD is hard coded. I'm working on a way of getting this out into an environment variable, but haven't figured it out yet. If you're forking this repo, you'll get my API key. Ideally, you should sign up for your own.  As soon as I figure out how to get mine out, it will disappear and no longer be available in this repo.

You can get your own API key by going to the [NASA API](https://api.nasa.gov/) page and following the instructions.

## Installing Rust

To install Rust, follow the [official instructions](https://rust-lang.org/tools/install) Remember, the minimum supported version for Yew is 1.56.1

## Install WebAssembly Target

The compilation target for browser-based WebAssembly is called wasm32-unknown-unknown.  You need to install it to your desktop development environment.

```
rustup target add wasm32-unknown-unknown
```

## Install Trunk

Trunk is a wonderful tool for building, deploying and packaging web sites and apps made with Yew.  More information on this tool can be found on their official site: [Trunk](https://trunkrs.dev)

```
cargo install --locked trunk
```

## Clone this repo

Clone this repo to your own development environment. You can use it as a template to do your own thing.

## Overview

[Shuttle](https://shuttle.rs) added the ability to host static files. This means you can spin up a web server using the Rust Axum crate, and use it to host a website on the Shuttle service.  Shuttle is in alpha right now, so bear that in mind. You shouldn't be putting mission critical stuff there just yet. It's great for hobbyist developers such as myself and I like to support this project.

...more to come...
