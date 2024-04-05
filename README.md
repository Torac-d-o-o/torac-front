# torac-front

This project holds the software used in the olive refinment factory, being composed of a [Sveltekit](https://kit.svelte.dev/) + [TailwindCSS](https://tailwindcss.com/) for it's
front-end and [Tauri](https://tauri.app/) to compile and bundle as a desktop and mobile application.

## Setting-up your environment

### Windows

You can just install these programs and follow [Tauri](https://tauri.app/)'s documentation for it's dependencies, once successfully installed you are set to go.

1. [NodeJS](https://nodejs.org/en)
2. [yarn](https://classic.yarnpkg.com/en/docs/install#debian-stable)
3. [Tauri's prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Linux

There are two routes with Linux: Either you follow the same steps from Windows, or use [nix](https://nixos.org/download/) and just run `nix develop`, or `nix-shell` if flakes are not enabled, to have an environment ready to go.

## Running the project

The project has three main gateways:

1. `yarn dev`
2. `yarn tauri dev`
3. `yarn tauri build`

The first will just host the web application as a server to be served, you can then open it in your browser of choice.

Now the second will also run the web application, but use tauri's web engine to display the page, this is the
**preferred** method to develop the application.

For the latter as it implies, this compiles it into a binary or package to be used in a live system, this is done for
testing purposes or for an actual release build.
