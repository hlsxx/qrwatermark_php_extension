# qrwatermark_php
![QR Code](https://github.com/hlsxx/qrwatermark_php/blob/master/php/qrwatemark.png)

PHP extension for the qrwatermark library

Check more about [qrwatermark](https://github.com/hlsxx/qrwatermark) written in the Rust programming language

## Install a PHP extension
First locate active php.ini

`php --ini`

`php -i | grep extension_dir`

For e.g.: /usr/lib/php/20230831

`cp target/release/libqrwatermark.so /usr/lib/php/20230831/qrwatermark.so`

### Make symlinks CLI/Apache
`ln -s /etc/php/8.3/mods-available/qrwatermark.ini /etc/php/8.3/cli/conf.d/30-qrwatermark.ini`
`ln -s /etc/php/8.3/mods-available/qrwatermark.ini /etc/php/8.3/apache2/conf.d/30-qrwatermark.ini`

`sudo systemctl restart apache2`

Validate installed PHP extension
`php -m | grep qrwatermark`

## How to use it?
### PHP CLI command
cd php && php -d extension=/path_to/libqrwatermark_php.so example.php



