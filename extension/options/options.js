const adapterSel = document.getElementById('adapter');
const saveBtn = document.getElementById('save');

chrome.storage.local.get(['adapter'], ({ adapter }) => {
  if (adapter) adapterSel.value = adapter;
});

saveBtn.addEventListener('click', () => {
  chrome.storage.local.set({ adapter: adapterSel.value });
});
