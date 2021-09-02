import * as fs from 'fs';
import axios from 'axios';
import * as AdmZip from 'adm-zip';
import * as iconv from 'iconv-lite';
import * as parse from 'csv-parse/lib/sync';
import * as ProgressBar from 'progress';

fs.mkdir("./public", error => {
	if (error) {
		console.error("public/ is already exists");
	}
});

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
		const csv: string[][] = parse(file, {
			delimiter: ",",
			onRecord: (records) => {
				return records.slice(2, 9)
			}
		});
		const progressBar = new ProgressBar(":bar :percent", {
			total: csv.length
		});
		csv.forEach(row => {
			const zipCode = row[0];
			fs.mkdir("./public/" + zipCode.slice(0, 3), () => {
				return true;
			});
			const json = {
				zipCode: zipCode,
				pref: row[4],
				prefKana: row[1],
				city: row[5],
				cityKana: row[2],
				town: row[6],
				townKana: row[3]
			};
			fs.writeFile("./public/" + zipCode.slice(0, 3) + "/" + zipCode.slice(3, 7) + ".json", JSON.stringify(json), error => {
				if (error) {
					throw Error;
				} else {
					progressBar.tick();
				}
			});
		})

	});
});
