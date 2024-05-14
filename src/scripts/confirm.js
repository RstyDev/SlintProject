const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;
var venta;

async function stash_n_close(venta) {
  console.log(venta);
  return await invoke("stash_n_close", { "pos": venta });
}

async function cancelar_venta(venta) {
  console.log(venta);
  return await invoke("cancelar_venta", { "pos": venta });
}
async function close_window() {
  return await invoke("close_window");
}

//document.getElementById('si').addEventListener('click', () => {
//  stash_n_close(venta);
//});
//document.getElementById('no').addEventListener('click', () => {
//  close_window();
//});



const unlisten = await listen('get-venta', (pl) => {
  if (!venta) {
    console.log(pl);
    venta = pl.payload.pos;
    document.getElementById('botones').innerHTML = `<button class="boton" id="si">Si</button>
    <button class="boton" id="no">No</button>`;
    let msg = document.getElementById('msg');
    if (pl.payload.message == 'stash') {
      msg.innerHTML ='Quieres guardar la venta para mas adelante?';
        document.getElementById('si').addEventListener('click', () => {
          stash_n_close(venta);
        })
    } else if (pl.payload.message == 'cancelar venta') {
      msg.innerHTML = 'Quieres cancelar la venta?';
      document.getElementById('si').addEventListener('click', () => {
        cancelar_venta(venta)
      })
    }

    document.getElementById('no').addEventListener('click', () => {
      close_window();
    });
    document.addEventListener('keydown', (e) => {
      e.preventDefault();
      if (e.keyCode == 13) {
        if (pl.payload.message == 'stash') {
          stash_n_close(venta);
        } else if (pl.payload.message == 'cancelar venta') {
          cancelar_venta(venta)
        }

      }
      else if (e.keyCode == 27) {
        close_window();
      }
    })
  }
})
