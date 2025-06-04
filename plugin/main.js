/* CloakedCanvas UXP panel */
console.log('CloakedCanvas plugin loaded');

document.addEventListener('DOMContentLoaded', () => {
  const btn = document.getElementById('exportBtn');
  if (btn) {
    btn.addEventListener('click', () => {
      console.log('Export button clicked');
      // TODO: invoke encryption using the core wasm module
    });
  }
});
