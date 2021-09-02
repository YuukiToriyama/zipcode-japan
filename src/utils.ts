import * as fs from 'fs';
import axios from 'axios';
import * as AdmZip from 'adm-zip';
import * as iconv from 'iconv-lite';
import * as ProgressBar from 'progress';

export const fetchUrl = async (url: string): Promise<Buffer> => {
	const response = await axios.get(url, {
		responseType: 'arraybuffer'
	});
	return response.data;
}
export const unzipAndDecode = (buffer: Buffer): string => {
	const zip = new AdmZip(buffer);
	const entries = zip.getEntries();
	return iconv.decode(entries[0].getData(), 'Shift_JIS');
}
export const createOutputs = (parsedCsv: string[][]) => {
	const progressBar = new ProgressBar(":bar :percent", {
		total: parsedCsv.length
	});
	parsedCsv.forEach(async row => {
		const zipCode = row[0];
		await fs.promises.mkdir("./public/" + zipCode.slice(0, 3)).catch(error => {
			//console.error(error);
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
		fs.promises.writeFile("./public/" + zipCode.slice(0, 3) + "/" + zipCode.slice(3, 7) + ".json", JSON.stringify(json)).then(() => {
			progressBar.tick();
		}).catch(error => {
			console.log(zipCode);
			throw error;
		});
	})
}
