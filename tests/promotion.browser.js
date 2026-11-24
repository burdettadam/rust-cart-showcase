async (page) => {
  async function checkText(selector, expected) {
    const actual = (await page.locator(selector).innerText()).trim();
    if (actual !== expected) throw new Error(`${selector}: expected ${expected}, got ${actual}`);
  }

  await page.getByRole('button', { name: 'Run the example' }).click();
  const weetabix = page.getByRole('listitem').filter({ hasText: 'Weetabix' });
  await weetabix.getByText('1 free', { exact: true }).waitFor();
  await checkText('#merchandise-subtotal', '15.02');
  await checkText('#discount', '9.98');
  await checkText('#subtotal', '5.04');
  await checkText('#tax', '0.63');
  await checkText('#total', '5.67');

  await page.locator('#quantity-cornflakes').fill('3');
  await page.getByRole('button', { name: 'Add Cornflakes to cart' }).click();
  await checkText('#discount', '12.50');
  await checkText('#total', '11.34');

  await page.getByRole('button', { name: 'Reset' }).click();
  await page.locator('#quantity-cornflakes').fill('2');
  await page.getByRole('button', { name: 'Add Cornflakes to cart' }).click();
  await checkText('#subtotal', '5.04');
  await page.getByRole('button', { name: 'Reset' }).click();
  await page.getByRole('button', { name: 'Add Weetabix to cart' }).click();
  await checkText('#discount', '0.00');
  await checkText('#total', '11.23');
}
