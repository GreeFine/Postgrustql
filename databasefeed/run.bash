#!/bin/bash
while [ true ]
do
  PERPAGE=50
  COUNT=$(cat './flowers/REQUEST_ID')
  echo Starting from $COUNT

  FILE="./flowers/$COUNT.json"
  if [ -f "$FILE" ]; then
    echo $FILE Already Exist
    exit
  fi
  # URL='https://api.tela-botanica.org/service:del:0.1/images?navigation.depart='$COUNT'&navigation.limite=50';
  URL='https://api.tela-botanica.org/service:del:0.1/images?navigation.depart='$COUNT'&navigation.limite='$PERPAGE'&masque.referentiel=bdtfx&masque.pninscritsseulement=1&protocole=1'

  echo $URL
  curl $URL > $FILE
  echo $FILE filled

  ((COUNT+=PERPAGE))
  echo $COUNT > ./flowers/REQUEST_ID;

  node index.js $FILE

  sleep 1200
done