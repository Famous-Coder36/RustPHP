# RustPHP
Rust engine for php

## Install

composer require famouscoder/rustphp

## Usage
php -d extension=./vendor/famouscoder/rustphp/ext/libssalom.so test.php
```php
<?php
require "vendor/autoload.php";

use RustPHP\Engine;

Engine::println("salom");

```