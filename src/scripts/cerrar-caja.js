const { invoke } = window.__TAURI__.tauri;
const bills=[1,2,5,10,20,50,100,200,500,1000,2000]
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}
async function cerrar_caja(monto_actual) {
    return await invoke("cerrar_caja", {montoActual:monto_actual});
}
async function get_caja(){
    return await invoke("get_caja")
}
function getRow(id){
    return id*document.getElementById(''+id).value;
}
get_caja().then(caja =>{
    let info=document.getElementById('info');
    let times=caja.inicio.split('T');
    info.innerHTML+=`<p>Inicio de caja: ${times[0]}</p>
    <p>Total de ventas: ${caja.ventas_totales}</p>`
    console.log(caja);
})
document.getElementsByTagName('form')[0].addEventListener('submit',(e)=>{
    e.preventDefault();
    let sum=0;
    for (let i=0;i<bills.length;i++){
        sum+=getRow(bills[i]);
    }
    cerrar_caja(sum)
})

//cierre: null
//id: 0
//inicio: "2024-02-16T20:16:11.714136337"
//monto_cierre: null
//monto_inicio: 0
//ventas_totales: 0