import { useEffect, useState } from "react";
import "./ProductoBusqueda.css";
import { invoke } from "@tauri-apps/api/tauri";
async function get_descripcion_valuable(prod, conf) {
    return await invoke("get_descripcion_valuable", { "prod": prod, "conf": conf });
}
async function agregarProdVentaAct(prod,pos) {
    return await invoke("agregar_producto_a_venta", { prod: prod, pos: pos });
}
function ProductoBusqueda({conf,producto,focused,valor,setFocuseado,index,pos,draw,prod}){
    const [desc,setDesc] = useState(prod);
    useEffect(()=>{setDesc(prod)},[prod])
    console.log(producto)
    
    return(<tr tabIndex="2" id={index} onClick={()=>{setFocuseado(index)}} onDoubleClick={()=>{agregarProdVentaAct(producto,pos);draw(true)}} className={focused}>
        <td className={conf.modo_mayus}>{desc}</td>
        <td>${valor}</td>
    </tr>)
}


export default ProductoBusqueda;