;; GAFRO Extended - Project-specific Emacs Configuration
;; This file provides project-specific settings for both vanilla Emacs and Spacemacs

((c++-mode
  . (     (c-basic-offset . 2)
     (tab-width . 2)
     (indent-tabs-mode . nil)
     (c-default-style . "stroustrup")
     (c-file-style . "stroustrup")
     (flycheck-clang-include-path
      . ("src"
         "src/gafro"
         "src/gafro/modern"
         "src/gafro/algebra"
         "src/gafro_control"))
     (flycheck-clang-definitions
      . ("GAFRO_EXTENDED"
         "GAFRO_USE_TAU"))
     (flycheck-clang-standard-library . "c++23")
     (flycheck-clang-language-standard . "c++23")
     (company-clang-arguments
      . ("-std=c++23"
         "-I./src"
         "-I./src/gafro"
         "-I./src/gafro/modern"
         "-I./src/gafro/algebra"
         "-I./src/gafro_control"
         "-DGAFRO_EXTENDED"
         "-DGAFRO_USE_TAU"))
     (lsp-clients-clangd-args
      . ("--compile-commands-dir=build"
         "--header-insertion=never"
         "--completion-style=detailed"
         "--function-arg-placeholders=false"))
     (projectile-project-compilation-cmd . "cd build && make")
     (projectile-project-test-cmd . "cd build && make test")))

 (cmake-mode
  . ((cmake-tab-width . 2)))

 (markdown-mode
  . ((markdown-header-scaling . t)
     (markdown-fontify-code-blocks-natively . t)))

 (rust-mode
  . ((rust-indent-offset . 2)
     (tab-width . 2)
     (indent-tabs-mode . nil))))

;; Project root detection (works with both vanilla Emacs and Spacemacs)
((nil . ((projectile-project-root . ".")
         (projectile-project-name . "gafro-extended")
         (projectile-project-type . 'cmake)
         (eval . (progn
                   ;; Spacemacs-specific settings
                   (when (boundp 'spacemacs-mode)
                     (setq-local spacemacs-default-layout-name "GAFRO")
                     (setq-local spacemacs-default-layout-binding "g"))
                   ;; Ensure LSP uses the correct compile commands
                   (when (boundp 'lsp-clients-clangd-args)
                     (setq-local lsp-clients-clangd-args
                                 '("--compile-commands-dir=build"
                                   "--header-insertion=never"
                                   "--completion-style=detailed"
                                   "--function-arg-placeholders=false")))))))
