const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;
var venta;

async function stash_n_close(venta) {
  return await invoke("stash_n_close", { "pos": venta });
}

async function close_window() {
  return await invoke("close_window");
}

document.addEventListener('keydown',(e)=>{
  e.preventDefault();
  if (e.keyCode==13){
    stash_n_close(venta);
  }
  else if (e.keyCode==27){
    close_window();
  }
})

const unlisten = await listen('get-venta', (pl) => {
  if (!venta){
    console.log(pl);
    venta=pl.payload.pos;
    document.getElementById('botones').innerHTML=`<button class="boton" id="si">Si</button>
    <button class="boton" id="no">No</button>`;
    console.log(pl);
    document.getElementById('si').addEventListener('click',()=>{
      stash_n_close(venta);
    });
    document.getElementById('no').addEventListener('click',()=>{
      close_window();
    });
  }
})
