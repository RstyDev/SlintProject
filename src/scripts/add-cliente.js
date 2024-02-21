const { invoke } = window.__TAURI__.tauri;
let rango;
let nombre=document.getElementById('nombre');
let dni=document.getElementById('dni');
let cred=document.getElementById('cred');
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
});
get_rango().then(r=>{
    rango=r;
    if (rango!='Admin'){
        document.getElementById('cred').disabled='true'
    }else{
        let aux=cred.value;
        console.log(aux)
    }
})
async function get_rango(){
    return await invoke("get_rango");
}
async function close_window() {
    return await invoke("close_window");
}
document.getElementsByClassName('add-form')[0].addEventListener('submit',(e)=>{
    e.preventDefault();
    let credito;
    if (rango=='Admin'){
        credito=cred.value=='on';
        
    }else{
        credito=false;
    }
    try{
        agregar_cliente(nombre.value,dni.value,credito).then(cli=>{
            console.log(cli)
        });
    }catch (error){
        console.error(error);
    }
})
async function agregar_cliente(nombre,dni,credito){
    return await invoke("agregar_cliente", { nombre:nombre, dni:parseInt(dni), credito:credito})
}