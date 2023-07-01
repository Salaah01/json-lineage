var fs = require('fs');
var filePath = "/home/salaah/json-lineage/sample_data/sample.json";
var stream = fs.createReadStream(filePath, 'utf8');
stream.on('data', function (chunk) {
    var json = JSON.parse(chunk);
    console.log(json);
});
stream.on('error', function (err) {
    console.error('Error reading file:', err);
});
