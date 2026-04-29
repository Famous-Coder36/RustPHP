<?php

namespace RustPHP;

class Engine extends \RustEngine
{
    public function __construct()
    {
        $this->loadExtension();
    }

    private function loadExtension(): void
    {
        if (!extension_loaded("rustphp")) {

            $ext = __DIR__ . "/vendor/famouscoder/rustphp/target/debug/librustphp.so";

            if (!file_exists($ext)) {
                throw new \Exception("❌ Extension file not found: $ext");
            }

            
            if (!function_exists("dl")) {
                throw new \Exception(
                    "❌ Rust extension is not loaded!\n" .
                    "👉 Add to php.ini: extension=$ext"
                );
            }

            dl($ext);
        }
    }

    public function echo($text): void
    {
        $this->print((string)$text);
    }

    public function echoln($text): void
    {
        $this->println((string)$text);
    }
}

class File extends \FileEngine
{
}

class Tgbot extends \TelegramBot
{
}

class Http extends \HttpClient
{
}

class Rglob extends \Request
{
}

class MySQL extends \DB
{
}

class Rayon extends \RayonClass
{
}