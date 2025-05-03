import fs from 'fs';
import path from 'path';

async function main() {
	const year = 2031;
	let counter = 0;
	let error = 0;

	const countriesFile = path.join(__dirname, 'country_coverage.json');
	const outputDir = path.join(__dirname, `holidays_${year}`);

	// Create output folder if it doesn't exist
	if (!fs.existsSync(outputDir)) {
		fs.mkdirSync(outputDir);
	}

	// Read country coverage file
	const countriesData = JSON.parse(fs.readFileSync(countriesFile, 'utf-8'));

	for (const country of countriesData) {
		if (!country.available) continue; // Skip unavailable countries

		const { code, name } = country;
		const url = `https://date.nager.at/api/v3/PublicHolidays/${year}/${code}`;

		try {
			console.log(`Fetching holidays for ${name} (${code})...`);
			const res = await fetch(url);

			if (!res.ok) {
				console.error(`Failed to fetch ${code}: ${res.status} ${res.statusText}`);
				error++;
				continue;
			}

			const holidays = await res.json();
			const safeName = name.replace(/[^a-z0-9]/gi, '_'); // Replace spaces/special characters

			const outputFile = path.join(outputDir, `${year}-${code}-${safeName}.json`);
			fs.writeFileSync(outputFile, JSON.stringify(holidays, null, 2));

			console.log(`Saved to ${outputFile}`);
			counter++;
		} catch (error) {
			console.error(`Error fetching ${code}:`, error);
			error++;
		}
	}

	console.log('Finished fetching all holidays.');
	console.log(`${counter} fetched successfully, ${error} failed`);
}

main();
