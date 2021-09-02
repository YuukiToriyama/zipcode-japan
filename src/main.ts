import axios from 'axios';
import * as AdmZip from 'adm-zip';
import * as iconv from 'iconv-lite';
import * as parse from 'csv-parse/lib/sync';

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
		const file = iconv.decode(entry.getData(), 'Shift_JIS');
		console.log(file);
		console.log(entry.entryName);
		console.log(buffer.length);

		const csv = parse(file, {
			delimiter: ",",
			onRecord: (records) => {
				return records.slice(2, 9)
			}
		});
		console.log(csv);
	});
});