import init, { hello, generate_qr_svg } from '@qrcode/wasm';

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
      ${generate_qr_svg()}
    </div>
  `;
}

main().catch(console.error); 