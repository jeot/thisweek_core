const puppeteer = require('puppeteer');
const fs = require('fs');
const path = require('path');

(async () => {
	const year = process.argv[2] || "1404";

	const browser = await puppeteer.launch({ headless: true });
	const page = await browser.newPage();

	await page.goto(`https://www.time.ir/fa/eventyear-%D8%AA%D9%82%D9%88%DB%8C%D9%85-%D8%B3%D8%A7%D9%84%DB%8C%D8%A7%D9%86%D9%87`, {
		waitUntil: 'networkidle2',
	});

	console.log("✅ Page loaded");

	// Clear input and type year slowly
	await page.focus("input[type='text']");
	await page.click("input[type='text']", { clickCount: 3 });
	await page.keyboard.press("Backspace");
	await page.type("input[type='text']", year, { delay: 100 });

	// Submit form and wait for full reload
	await Promise.all([
		page.click("input[type='submit']"),
		page.waitForNavigation({ waitUntil: 'networkidle2' }),
	]);

	console.log("✅ Page reloaded for year", year);

	// 🔍 Check year correctness
	const pageYear = await page.evaluate(() => {
		const persianDigits = '۰۱۲۳۴۵۶۷۸۹';
		const faToEn = s => s.replace(/[۰-۹]/g, d => persianDigits.indexOf(d));
		const text = document.querySelector("div.dates > span > a.jalali")?.innerText || "";
		const enText = faToEn(text);
		const match = enText.match(/\d{4}/);
		return match ? match[0] : null;
	});

	if (pageYear !== year) {
		console.error(`❌ Mismatch: page year is ${pageYear}, expected ${year}`);
		await browser.close();
		process.exit(1);
	}

	console.log("🔍 Collecting holiday data...");

	const holidays = await page.evaluate((year) => {
		const persianDigits = '۰۱۲۳۴۵۶۷۸۹';
		const faToEn = s => s.replace(/[۰-۹]/g, d => persianDigits.indexOf(d));

		const monthElements = Array.from(document.querySelectorAll("div.col-md-12 > div > div > span > span > span"));
		const monthNames = monthElements.map(el => el.innerText.trim());

		return Array.from(document.querySelectorAll("li.eventHoliday")).map(node => {
			const parts = node.innerText.trim().split(/\s+/);
			if (parts.length < 3) return null;

			const day = faToEn(parts[0]).padStart(2, '0');
			const monthName = parts[1];
			const desc = parts.slice(2).join(" ");
			const monthIndex = (monthNames.findIndex(x => x === monthName) + 1).toString().padStart(2, '0');

			return {
				date: `${year}-${monthIndex}-${day}`,
				localName: desc,
				name: desc,
				countryCode: "IR"
			};
		}).filter(Boolean);
	}, year);

	if (holidays.length) {
		const outputPath = path.resolve(__dirname, `${year}-IR-Iran.json`);
		fs.writeFileSync(outputPath, JSON.stringify(holidays, null, 2), 'utf-8');
		console.log(`✅ Successfully saved ${holidays.length} items to ${outputPath}`);
	} else {
		console.error("❌ No holidays found or parsing failed.");
	}

	await browser.close();
})();
