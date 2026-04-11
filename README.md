# RustPHP
Rust engine for php

## Install

composer require famouscoder/enginer

## Usage
php -d extension=./vendor/famouscoder/enginer/ext/libssalom.so test.php
```php
<?php
require "vendor/autoload.php";

use RustPHP\Engine;

Engine::println("salom");

```