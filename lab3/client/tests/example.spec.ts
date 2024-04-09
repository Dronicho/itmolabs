import { test, expect } from '@playwright/test';

test('registration', async ({ page }) => {
  await page.goto('/register');

  await page.fill('input[name="email"]', generateRandomEmail());
  await page.fill('input[name="password"]', '12345');
  await page.fill('input[name="name"]', 'test');
  await page.click('button[type="submit"]');

  await page.waitForURL('**/convert');
});

function generateRandomEmail() {
  const chars = 'abcdefghijklmnopqrstuvwxyz1234567890';
  let email = '';
  for (let i = 0; i < 10; i++) {
    email += chars[Math.floor(Math.random() * chars.length)];
  }
  email += '@example.com';
  return email;
}

test('convert', async ({ page }) => {
  await page.goto('/login');

  await page.fill('input[name="email"]', 'a@a.ru ');
  await page.fill('input[name="password"]', '12345');
  await page.click('button[type="submit"]');

  await page.waitForURL('**/convert');

  const fileChooserPromise = page.waitForEvent('filechooser');
  await page.getByTestId('dropzone').click();
  const fileChooser = await fileChooserPromise;
  await fileChooser.setFiles('tests/test.mp4');

  await page.waitForSelector('a');
});
