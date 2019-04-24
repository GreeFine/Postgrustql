if (process.argv[2] == undefined) {
  console.log('Missing file name');
  process.exit();
}

var mysql = require('mysql');
var connection = mysql.createConnection({
  host: 'localhost',
  user: 'greefine',
  password: 'password',
  database: 'Flowers',
});
connection.connect();

const fs = require('fs');

let rawdata = fs.readFileSync(process.argv[2]);
let data = JSON.parse(rawdata);

for (var res in data.resultats) {
  res = data.resultats[res];
  var querry_fields =
    '(id_image, binaire_href, mots_cles_texte, id_observation, date_observation, date_transmission, determination_famille, determination_ns, determination_nn, determination_nt, determination_referentiel, id_zone_geo, zone_geo, lieudit, station, milieu, pays, hauteur, date, nom_original)';
  var querry_values =
    '("' +
    res['id_image'] +
    '", "' +
    res['binaire.href'] +
    '", "' +
    res['mots_cles_texte'] +
    '", "' +
    res.observation['id_observation'] +
    '", "' +
    res.observation['date_observation'] +
    '", "' +
    res.observation['date_transmission'] +
    '", "' +
    res.observation['determination.famille'] +
    '", "' +
    res.observation['determination.ns'] +
    '", "' +
    res.observation['determination.nn'] +
    '", "' +
    res.observation['determination.nt'] +
    '", "' +
    res.observation['determination.referentiel'] +
    '", "' +
    res.observation['id_zone_geo'] +
    '", "' +
    res.observation['zone_geo'] +
    '", "' +
    res.observation['lieuditres.observation'] +
    '", "' +
    res.observation['stationres.observation'] +
    '", "' +
    res.observation['milieu'] +
    '", "' +
    res.observation['pays'] +
    '", "' +
    res.observation['hauteur'] +
    '", "' +
    res.observation['date'] +
    '", "' +
    res.observation['nom_original'] +
    '");';
  connection.query(
    'INSERT INTO pictures ' + querry_fields + ' VALUES ' + querry_values,
    function(error, results, fields) {
      if (error) console.log(error, results);
    }
  );
}

connection.end();
