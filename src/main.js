"use strict";
exports.__esModule = true;
var node_fetch_1 = require("node-fetch");
var AdmZip = require("adm-zip");
var url = "https://www.post.japanpost.jp/zipcode/dl/kogaki/zip/26kyouto.zip";
(0, node_fetch_1["default"])(url).then(function (response) {
    console.log(url);
    return response.buffer();
}).then(function (buffer) {
    var zip = new AdmZip(buffer);
    var entries = zip.getEntries();
    entries.forEach(function (entry) {
        var file = entry.getData();
        console.log(entry.entryName);
        console.log(buffer.length);
    });
});
