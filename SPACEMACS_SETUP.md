# Spacemacs Setup for GAFRO Extended

This file provides suggestions for configuring Spacemacs to work optimally with GAFRO Extended.

## Required Layers

Add these layers to your `~/.spacemacs` file in the `dotspacemacs-configuration-layers` section:

```elisp
(c-c++ :variables
       c-c++-default-mode-for-headers 'c++-mode
       c-c++-enable-clang-support t
       c-c++-enable-google-style t
       c-c++-enable-google-newline t
       c-c++-enable-c++11 t
       c-c++-enable-c++14 t
       c-c++-enable-c++17 t
       c-c++-enable-c++20 t
       c-c++-enable-c++23 t)

lsp
git
markdown
cmake
rust
auto-completion
syntax-checking
helm
format-all
```

## User Configuration

Add this to your `dotspacemacs/user-config` section in `~/.spacemacs`:

```elisp
;; GAFRO Extended specific configuration
(setq-default c-basic-offset 2)
(setq-default tab-width 2)
(setq-default indent-tabs-mode nil)
(setq-default c-default-style "stroustrup")

;; LSP configuration for C++
(with-eval-after-load 'lsp-mode
  (setq lsp-clients-clangd-args
        '("--compile-commands-dir=build"
          "--header-insertion=never"
          "--completion-style=detailed"
          "--function-arg-placeholders=false")))

;; Custom key bindings for GAFRO development
(spacemacs/set-leader-keys
  "cb" 'gafro-build-project
  "ct" 'gafro-test-project
  "cc" 'gafro-clean-build
  "cf" 'clang-format-region
  "cr" 'gafro-run-rust-tests)

;; Custom functions for GAFRO development
(defun gafro-build-project ()
  "Build the GAFRO project"
  (interactive)
  (compile "cd build && make"))

(defun gafro-test-project ()
  "Run GAFRO tests"
  (interactive)
  (compile "cd build && make test"))

(defun gafro-clean-build ()
  "Clean and rebuild the project"
  (interactive)
  (compile "cd build && make clean && make"))

(defun gafro-run-rust-tests ()
  "Run Rust tests"
  (interactive)
  (compile "cd shared_tests/rust && cargo test"))
```

## Key Bindings

- `SPC c b`: Build project
- `SPC c t`: Run tests  
- `SPC c c`: Clean and rebuild
- `SPC c f`: Format code with clang-format
- `SPC c r`: Run Rust tests

## Setup Steps

1. **Build the project** to generate `compile_commands.json`:

   ```bash
   mkdir build && cd build
   cmake ..
   make
   ```

2. **Restart Spacemacs** after adding the configuration

3. **Open a C++ file** in the project - LSP should automatically start

## Features

- **LSP Integration**: Full language server support with clangd
- **Auto-completion**: Intelligent C++ completion
- **Syntax checking**: Real-time error detection
- **Project navigation**: Quick file and symbol navigation
- **Build integration**: One-key build and test commands
- **Code formatting**: clang-format integration

The `.dir-locals.el` file in this repository will automatically configure project-specific settings when you open files in this directory.
