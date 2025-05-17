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
      <input id="qr-input" type="text" placeholder="Enter a URL" value="https://google.com" />
      <button id="qr-generate">Generate QR Code</button>
      <div id="qr-output"></div>
    </div>
  `;

  const input = document.getElementById('qr-input') as HTMLInputElement | undefined;
  const button = document.getElementById('qr-generate') as HTMLButtonElement | undefined;
  const output = document.getElementById('qr-output') as HTMLDivElement | undefined;

  function updateQR() {
    if (!input || !output) {
      console.error('setup failure', !input ? 'input not found' : '', !output ? 'output not found' : "")
    } else {
    const url = input.value.trim();
    if (!url) {
      output.innerHTML = "<em>Please enter a URL.</em>";
      return;
    }
    try {
      const svg = generate_qr_svg(url);
      output.innerHTML = svg;
    } catch (e) {
      output.innerHTML = "<em>Failed to generate QR code.</em>";
      console.error(e);
    }
    }
  }

  if (button) {
    button.addEventListener('click', updateQR)
  } else {
    console.error('setup failre', 'button not found')
  }


}

main().catch(console.error); 