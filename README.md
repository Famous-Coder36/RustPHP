# RustPHP
Rust engine for php

## Requirements
• PHP 8.2+
• Rust

## Composer
composer.json:
```json
{
  "repositories": [
    {
      "type": "vcs",
      "url": "https://github.com/Famous-Coder36/RustPHP.git"
    }
  ],
  "require": {
    "famouscoder/rustphp": "*"
  }
}
```

## Install
```bash
composer require famouscoder/rustphp
cargo build --manifest-path=vendor/famouscoder/rustphp/Cargo.toml
```

## Exemple

test.php:
```php
<?php
require "vendor/autoload.php";

use RustPHP\Engine;

Engine::println("salom");

```

## Usage

```bash
php -d extension=./vendor/famouscoder/rustphp/target/debug/libssalom.so test.php
```