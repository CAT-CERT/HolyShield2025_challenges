const puppeteer = require('puppeteer');
const fs = require('fs').promises;
require('dotenv').config({ path: __dirname + '/../config/.env' });

let flag;

const check = async (name) => {
  const baseUrl = `http://${process.env.EXTERNAL_HOST}:${process.env.EXTERNAL_PORT}`;
  const url = `${baseUrl}/devil/`;
  const loginPageUrl = `${baseUrl}/auth/login`;

  const launchOptions = {
    headless: 'new',
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--disable-dev-shm-usage',
      '--disable-gpu',
      '--disable-extensions',
      '--disable-background-networking',
      '--disable-sync',
      '--disable-default-apps',
      '--disable-translate',
      '--metrics-recording-only',
      '--no-first-run',
      '--no-zygote'
    ],
    timeout: 20000
  };

  let browser;

  try {
    if (!flag) {
      flag = (await fs.readFile('/flag.txt', 'utf8')).trim();
      await fs.unlink('/flag.txt');
    }

    browser = await puppeteer.launch(launchOptions);
    const page = await browser.newPage();

    await page.goto('about:blank');

    await page.setRequestInterception(true);
    page.on('request', req => {
      const type = req.resourceType();
      if (type === 'image' || type === 'font' || type === 'media') {
        req.abort();
      } else {
        req.continue();
      }
    });

    await page.setCookie({
      name: 'FLAG',
      value: flag,
      url: baseUrl,
      path: '/',
      httpOnly: false,
      secure: false,
      sameSite: 'Lax'
    });

    await page.goto(loginPageUrl, {
      waitUntil: 'domcontentloaded',
      timeout: 8000
    });

    await page.waitForSelector('input[type="text"]', { timeout: 4000 });
    await page.waitForSelector('input[type="password"]', { timeout: 4000 });

    await page.type('input[type="text"]', 'admin', { delay: 10 });
    await page.type('input[type="password"]', 'a83kd91kf0gsd93kv8s82k3l49d', { delay: 10 });

    await page.click('button[type="submit"]');

    await page.waitForFunction(
      () => !document.querySelector('button[type="submit"]'),
      { timeout: 6000 }
    ).catch(() => {});

    await page.goto(url, {
      waitUntil: 'domcontentloaded',
      timeout: 8000
    });

    const selector = `#devil-${name}`;
    await page.waitForSelector(selector, { timeout: 4000 });
    await page.$eval(selector, el => el.scrollIntoView({ block: 'center' }));
    const prevPath = await page.evaluate(() => location.pathname);
    await page.click(selector);
    await Promise.race([
      page.waitForFunction(
        prev => location.pathname !== prev,
        { timeout: 8000 },
        prevPath
      ),
      page.waitForSelector('.detail-card', { timeout: 8000 }),
      page.waitForSelector('.loading', { timeout: 8000 })
    ]).catch(() => {});

    return { success: true };
  } catch (err) {
    console.error('[check.service] error:', err?.stack || err);
    throw err;
  } finally {
    if (browser) {
      try {
        await browser.close();
      } catch (_) {}
    }
  }
};

module.exports = check;
