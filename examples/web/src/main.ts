import init, { hello } from '@qrcode/wasm';

async function main() {
  // Initialize the WASM module
  await init();

  // Call the hello function
  const message = hello();

  // Update the UI
  document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <div>
      <h1>QR Code Generator</h1>
      <p>${message}</p>
    </div>
  `;
}

main().catch(console.error); 