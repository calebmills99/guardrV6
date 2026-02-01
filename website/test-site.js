const { chromium } = require('playwright');

(async () => {
  console.log('Launching browser...');
  const browser = await chromium.launch({ headless: false });
  const context = await browser.newContext();
  const page = await context.newPage();
  
  // Listen for console messages
  page.on('console', msg => console.log('BROWSER:', msg.text()));
  
  // Listen for network requests
  page.on('request', request => {
    if (request.url().includes('/check')) {
      console.log('>> API REQUEST:', request.method(), request.url());
    }
  });
  
  page.on('response', response => {
    if (response.url().includes('/check')) {
      console.log('<< API RESPONSE:', response.status(), response.url());
    }
  });

  console.log('Opening website at http://127.0.0.1:9999 ...');
  await page.goto('http://127.0.0.1:9999', { waitUntil: 'networkidle' });
  
  console.log('Page title:', await page.title());
  
  // Take screenshot of initial page
  await page.screenshot({ path: 'C:\\runb\\guardrV6\\website\\test-1-loaded.png', fullPage: true });
  console.log('✅ Page loaded - screenshot saved');
  
  // Scroll to demo section
  await page.evaluate(() => window.scrollTo(0, 800));
  await page.waitForTimeout(500);
  
  // Wait for the form
  await page.waitForSelector('input[type="text"]', { timeout: 10000 });
  console.log('✅ Form found');
  
  // Fill in the demo form
  console.log('Filling form: Caleb Stewart, Los Angeles');
  const inputs = await page.$$('input[type="text"]');
  await inputs[0].fill('Caleb Stewart');
  if (inputs[1]) await inputs[1].fill('Los Angeles');
  
  await page.screenshot({ path: 'C:\\runb\\guardrV6\\website\\test-2-filled.png', fullPage: true });
  console.log('✅ Form filled - screenshot saved');
  
  // Click submit
  console.log('Clicking submit button...');
  await page.click('button[type="submit"]');
  
  // Wait for API response (look for loading state to finish or results to appear)
  console.log('Waiting for API response (up to 30s)...');
  try {
    await page.waitForSelector('text=Safety Report', { timeout: 30000 });
    console.log('✅ GOT RESULTS!');
    // Wait extra time for UI to fully render
    await page.waitForTimeout(2000);
  } catch (e) {
    console.log('⚠️ Timeout waiting for results, checking page state...');
  }
  
  await page.screenshot({ path: 'C:\\runb\\guardrV6\\website\\test-3-result.png', fullPage: true });
  console.log('✅ Final screenshot saved');
  
  // Get any visible text about risk
  const pageText = await page.textContent('body');
  if (pageText.includes('Risk')) {
    console.log('✅ Risk information found on page');
  }
  if (pageText.includes('error') || pageText.includes('Error')) {
    const errorMatch = pageText.match(/error[^.]*\./i);
    if (errorMatch) console.log('❌ Error found:', errorMatch[0]);
  }
  
  // Keep browser open
  console.log('Browser will stay open for 15 seconds...');
  await page.waitForTimeout(15000);
  
  await browser.close();
  console.log('Done!');
})();
