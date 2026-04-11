# RustPHP
Rust engine for php

## Install

composer require famouscoder/rustphp

cargo build --manifest-path=vendor/famouscoder/rustphp/Cargo.toml

## Usage
```bash
php -d extension=./vendor/famouscoder/rustphp/target/debug/libssalom.so test.php
```
```php
<?php
require "vendor/autoload.php";

use RustPHP\Engine;

Engine::println("salom");

```