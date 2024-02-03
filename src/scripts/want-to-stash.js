
const { emit, listen } = window.__TAURI__.event;

const { invoke } = window.__TAURI__.tauri;

console.log("igual aca");
// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
const unlisten = await listen('get-venta', (pl) => {
  console.log('pl');
})