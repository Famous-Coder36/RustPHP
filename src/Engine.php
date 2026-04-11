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
        if (!extension_loaded("ssalom")) {

            $ext = __DIR__ . "/../ext/libssalom.so";

            if (!file_exists($ext)) {
                throw new \Exception("❌ Extension file not found: $ext");
            }

            if (function_exists("dl")) {
                dl($ext);
            } else {
                throw new \Exception(
                    "❌ extension yuklanmagan!\n" .
                    "👉 php.ini ga qo‘sh: extension=libssalom.so"
                );
            }
        }
    }


}
