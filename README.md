# QrWatermark PHP extension

This is a Rust-based PHP extension that provides QR code creation with watermark support.

Recommended used with official PHP connector

```
composer require hlsxx/qrwatermark
```

More about: https://github.com/hlsxx/qrwatermark_php

## Simple test with CLI
From the current root dir:

```
php -d extension=/path_to/qrwatermark.so php/example.php
```

## Installation

1. Download the `.so` file from the [releases](https://github.com/yourusername/my_php_extension/releases) section or directly from this repository.
2. Copy the `.so` file to your PHP extensions directory. The directory can typically be found by running `php -i | grep extension_dir`.
3. Add the following line to your `php.ini` file:

```ini
extension=qrwatermark.so
```

### Installation hints

First locate an active php.ini

```
php --ini
```

```
php -i | grep extension_dir
```

For e.g.: /usr/lib/php/20200831

```
cp target/release/libqrwatermark.so /usr/lib/php/20200831/qrwatermark.so
```

Create symlins CLI/Apache

```
ln -s /etc/php/8.3/mods-available/qrwatermark.ini /etc/php/8.3/cli/conf.d/30-qrwatermark.ini
ln -s /etc/php/8.3/mods-available/qrwatermark.ini /etc/php/8.3/apache2/conf.d/30-qrwatermark.ini
```

```
sudo systemctl restart apache2
````

Validate installed PHP extension
```
php -m | grep qrwatermark
```

Check more about [qrwatermark](https://github.com/hlsxx/qrwatermark) written in the Rust programming language
