const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;
let ya = false;
document.addEventListener('keydown', (e) => {
    if (e.keyCode == 27) {
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}

function dibujar(val) {
    let form = document.getElementsByClassName('add-form')[0];

    if (Object.keys(val) == 'Pes') {
        form.innerHTML = `
        <input type="number" step="0.01" name="cantidad" id="cantidad" placeholder="Cantidad" required>
        <button type="submit">Seleccionar</button>
        `
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            val.Pes[0] = parseFloat(document.getElementById('cantidad').value);
            console.log(val.Pes)
        })
    } else if (Object.keys(val) == 'Rub') {
        form.innerHTML = `
        <input type="number" name="monto" id="monto" placeholder="Monto" required>
        <button type="submit">Seleccionar</button>
        `
        form.addEventListener('submit', (e) => {
            e.preventDefault();
            val.Rub[1].monto = parseFloat(document.getElementById('monto').value);
            console.log(val.Rub[1])
        })
    }

}
const unlisten3 = await listen('select-amount', (pl) => {
    if (!ya) {
        dibujar(pl.payload.val)
        ya = true;
    }
})