<?php

$isQrWaterMarkGenerated = qrwatermark_generate(
  "TEST from PHP",
  "./php_logo.png",
  "./../qr_codes/qrwatemark.png"
);

if ($isQrWaterMarkGenerated) {
  echo "QRWatermark sucessfuly generated! \n";
  echo "Check qr_codes/qrwatermark.png";
} else {
  echo "ERROR: QRWatermark was not generated! \n";
  echo "Ensure that the qr_codes folder has enough permissions!";
}

?>
