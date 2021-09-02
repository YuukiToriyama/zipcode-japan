import axios from 'axios';
import * as AdmZip from 'adm-zip';

const url = "https://www.post.japanpost.jp/zipcode/dl/kogaki/zip/26kyouto.zip";
axios.get(url, {
	responseType: 'arraybuffer'
}).then(response => {
	console.log(url);
	return response.data;
}).then(buffer => {
	const zip = new AdmZip(buffer);
	const entries = zip.getEntries();
	entries.forEach(entry => {
		const file = entry.getData();
		console.log(entry.entryName);
		console.log(buffer.length);
	})
});