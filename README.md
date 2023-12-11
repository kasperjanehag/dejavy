# Dejavy App

Dejavy is a powerful and user-friendly application designed to help you clean up similar images from your file system. It's a handy tool that saves you time and storage space by identifying and removing duplicate images. Dejavy is cross-platform and can be used on Mac, PC, and Windows.

## Tech Stack

Dejavy is built using a robust and modern tech stack, carefully chosen to ensure performance, maintainability, and cross-platform compatibility.

### Languages

- **Rust:** A language empowering everyone to build reliable and efficient software. Rust is used in Tauri for creating the backend of the desktop application. It's memory-safe without garbage collection and has a rich type system, ensuring our app runs fast and reliably.

- **TypeScript:** A statically typed superset of JavaScript that adds optional types. TypeScript is used to catch errors early in development and enhance code quality and maintainability. It's widely supported across platforms and has excellent tooling, making it a great choice for cross-platform development.

### Frameworks and Libraries

- **Tauri:** A framework for building lightweight, secure, and cross-platform desktop applications with web technologies. Tauri is chosen for its low resource consumption and high performance. It allows us to use the same web technologies we're using for the rest of the app to build the desktop versions, ensuring consistency and reducing the amount of platform-specific code we need to write.

- **Vue 3:** A progressive JavaScript framework for building user interfaces. Vue 3 is used for its simplicity, reactivity system, and composition API. It's lightweight and flexible, making it a good fit for a performance-sensitive application like Dejavy. Vue's component-based architecture also makes it easy to build and maintain complex user interfaces.

- **Vuetify:** A Vue UI Library with beautifully handcrafted Material Components. Vuetify is used for its extensive library of pre-made components, which speeds up development and ensures a consistent look and feel across the app. It's built with Vue, so it integrates seamlessly with the rest of our Vue codebase.

- **Pinia:** State management for Vue. Pinia is used for its intuitive API and devtools support. It provides a centralized store for our app's state, making it easier to manage and debug. Pinia is designed to work with Vue, so it integrates well with our Vue components and reactivity system.

### Build Tool

- **Vite:** A next-generation frontend build tool. Vite is used for its fast hot module replacement (HMR) and on-demand file serving over native ES Modules. It's designed to work well with Vue and other modern web technologies, and it has excellent support for TypeScript. Vite's speed and flexibility make it a great fit for our development workflow.

- **Cargo:** Rust’s package manager. It downloads your Rust package’s dependencies, compiles your packages, makes distributable packages, and uploads them to crates.io. It's an essential tool for managing and building our Rust code.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/): A versatile and widely-used code editor with support for a multitude of programming languages.
- [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar): A VS Code extension for Vue.js development, providing features like IntelliSense, type checking, and more.
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode): A VS Code extension for Tauri, a framework for building lightweight, secure desktop applications with web technologies.
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer): A VS Code extension that provides a powerful IDE-like experience for the Rust programming language.

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
